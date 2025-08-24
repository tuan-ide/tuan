use crate::file::File;
use crate::graph_view::{Graph, GraphFeeder};
use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_ast_visit::Visit;
use oxc_parser::{Parser, ParserReturn};
use oxc_span::SourceType;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct TypescriptProject {
    project_path: PathBuf,
}

impl TypescriptProject {
    pub fn new(project_path: PathBuf) -> Self {
        Self { project_path }
    }
}

impl GraphFeeder for TypescriptProject {
    fn feed_graph(&self, graph: &mut Graph) {
        let root = &self.project_path;
        let ts_files = Self::find_typescript_files(root);

        for file in &ts_files {
            graph.add_file(file.clone());
        }

        for file in &ts_files {
            match Self::extract_typescript_imports(file) {
                Ok(imports) => {
                    for imported_file in imports {
                        graph.add_relation(file.clone(), imported_file);
                    }
                }
                Err(e) => {
                    tracing::error!(
                        "Error extracting imports from {}: {}",
                        file.path.display(),
                        e
                    );
                }
            }
        }
    }
}

impl TypescriptProject {
    fn find_typescript_files(root: &Path) -> Vec<File> {
        WalkDir::new(root)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_type().is_file()
                    && entry
                        .path()
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|ext| matches!(ext, "ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs"))
                        .unwrap_or(false)
            })
            .filter(|entry| {
                !entry.path().components().any(|component| {
                    // TODO: ignore files from .gitignore
                    matches!(
                        component.as_os_str().to_str(),
                        Some("node_modules")
                            | Some(".git")
                            | Some("dist")
                            | Some("build")
                            | Some("coverage")
                            | Some(".svelte-kit")
                    )
                })
            })
            .map(|entry| File::new(entry.path().to_path_buf()))
            .collect()
    }

    fn extract_typescript_imports(file: &File) -> Result<Vec<File>, Box<dyn std::error::Error>> {
        let source_code = fs::read_to_string(&file.path)?;
        let allocator = Allocator::default();

        let source_type = match file.path.extension().and_then(|s| s.to_str()) {
            Some("tsx") => SourceType::tsx(),
            Some("ts") => SourceType::ts(),
            Some("jsx") => SourceType::jsx(),
            Some("js") => SourceType::unambiguous(),
            Some("mjs") => SourceType::mjs(),
            Some("cjs") => SourceType::cjs(),
            _ => SourceType::default(),
        };

        let ParserReturn {
            program, errors, ..
        } = Parser::new(&allocator, &source_code, source_type).parse();

        if !errors.is_empty() {
            for error in &errors {
                eprintln!("Parse error in {}: {}", file.path.display(), error);
            }
        }

        let mut visitor = ImportVisitor::new(&file.path);
        visitor.visit_program(&program);

        Ok(visitor.imports)
    }
}

struct ImportVisitor {
    imports: Vec<File>,
    current_file_dir: PathBuf,
}

impl ImportVisitor {
    fn new(current_file_path: &PathBuf) -> Self {
        Self {
            imports: Vec::new(),
            current_file_dir: current_file_path
                .parent()
                .unwrap_or(Path::new(""))
                .to_path_buf(),
        }
    }

    fn add_import(&mut self, import_path: &str) {
        if let Some(resolved_path) = self.resolve_import_path(import_path) {
            let canonical_path = resolved_path.canonicalize().unwrap_or(resolved_path);
            self.imports.push(File::new(canonical_path));
        }
    }

    fn resolve_import_path(&self, import_path: &str) -> Option<PathBuf> {
        if import_path.starts_with('.') {
            let resolved = self.current_file_dir.join(import_path);

            let canonical_base = match resolved.canonicalize() {
                Ok(path) => path,
                Err(_) => self.manual_resolve_path(&resolved),
            };

            for ext in [".ts", ".tsx", ".js", ".jsx"] {
                let with_ext = canonical_base.with_extension(&ext[1..]);
                if with_ext.exists() {
                    return Some(with_ext);
                }
            }

            for ext in [".ts", ".tsx", ".js", ".jsx"] {
                let index_file = canonical_base.join(format!("index{}", ext));
                if index_file.exists() {
                    return Some(index_file);
                }
            }
        }
        None
    }

    fn manual_resolve_path(&self, path: &Path) -> PathBuf {
        use std::path::Component;
        let mut components = Vec::new();

        for component in path.components() {
            match component {
                Component::ParentDir => {
                    components.pop();
                }
                Component::CurDir => {}
                other => components.push(other),
            }
        }

        let mut result = PathBuf::new();
        for component in components {
            result.push(component);
        }
        result
    }
}

impl<'a> Visit<'a> for ImportVisitor {
    fn visit_import_declaration(&mut self, decl: &ImportDeclaration<'a>) {
        self.add_import(decl.source.value.as_str());
    }

    fn visit_export_all_declaration(&mut self, decl: &ExportAllDeclaration<'a>) {
        self.add_import(decl.source.value.as_str());
    }

    fn visit_export_named_declaration(&mut self, decl: &ExportNamedDeclaration<'a>) {
        if let Some(source) = &decl.source {
            self.add_import(source.value.as_str());
        }
    }

    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        if let Expression::ImportExpression(_) = &expr.callee {
            if let Some(first_arg) = expr.arguments.first() {
                if let Argument::StringLiteral(str_lit) = first_arg {
                    self.add_import(str_lit.value.as_str());
                }
            }
        }

        if let Expression::Identifier(ident) = &expr.callee {
            if ident.name == "require" {
                if let Some(first_arg) = expr.arguments.first() {
                    if let Argument::StringLiteral(str_lit) = first_arg {
                        self.add_import(str_lit.value.as_str());
                    }
                }
            }
        }

        self.visit_expression(&expr.callee);
        for arg in &expr.arguments {
            self.visit_argument(arg);
        }
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        if let MemberExpression::StaticMemberExpression(static_expr) = expr {
            if let Expression::CallExpression(call_expr) = &static_expr.object {
                if let Expression::Identifier(ident) = &call_expr.callee {
                    if ident.name == "require" {
                        if let Some(first_arg) = call_expr.arguments.first() {
                            if let Argument::StringLiteral(str_lit) = first_arg {
                                self.add_import(str_lit.value.as_str());
                            }
                        }
                    }
                }
            }
        }

        match expr {
            MemberExpression::ComputedMemberExpression(computed) => {
                self.visit_expression(&computed.object);
                self.visit_expression(&computed.expression);
            }
            MemberExpression::StaticMemberExpression(static_expr) => {
                self.visit_expression(&static_expr.object);
            }
            MemberExpression::PrivateFieldExpression(private) => {
                self.visit_expression(&private.object);
            }
        }
    }
}
