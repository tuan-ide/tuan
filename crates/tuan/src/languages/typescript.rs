use crate::file::File;
use crate::graph_view::{GraphFeeder, GraphState};
use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_ast_visit::Visit;
use oxc_parser::{Parser, ParserReturn};
use oxc_resolver::{ResolveOptions, Resolver, TsconfigOptions, TsconfigReferences};
use oxc_span::SourceType;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct TypescriptProject {
    project_path: PathBuf,
    resolver: Resolver,
}

impl TypescriptProject {
    pub fn new(project_path: PathBuf) -> Self {
        let resolver = Self::build_resolver(&project_path);
        Self {
            project_path,
            resolver,
        }
    }

    fn build_resolver(project_root: &Path) -> Resolver {
        // essaie de trouver un tsconfig à la racine (ou passe le chemin exact)
        let config_file = project_root.join("tsconfig.json");

        let options = ResolveOptions {
            // extensions gérées
            extensions: vec![
                ".ts".into(),
                ".tsx".into(),
                ".mts".into(),
                ".cts".into(),
                ".js".into(),
                ".jsx".into(),
                ".mjs".into(),
                ".cjs".into(),
                ".json".into(),
            ],
            // conditions "exports" (ESM) côté node
            condition_names: vec!["node".into(), "import".into()],
            // active la prise en compte de tsconfig (baseUrl, paths, extends, references)
            tsconfig: Some(TsconfigOptions {
                config_file,
                references: TsconfigReferences::Auto,
            }),
            ..ResolveOptions::default()
        };

        Resolver::new(options)
    }
}

impl GraphFeeder for TypescriptProject {
    fn feed_graph(&self, graph: &mut GraphState) {
        let root = &self.project_path;
        let ts_files = Self::find_typescript_files(root);
        println!(
            "{:?}",
            ts_files
                .iter()
                .map(|f| f.path.display())
                .collect::<Vec<_>>()
        );

        for file in &ts_files {
            graph.add_file(file.clone());
        }

        for file in &ts_files {
            match Self::extract_typescript_imports(file, &self.resolver, root) {
                Ok(imports) => {
                    for imported_file in imports {
                        graph.add_relation(file.clone(), imported_file);
                    }
                }
                Err(e) => tracing::error!(
                    "Error extracting imports from {}: {}",
                    file.path.display(),
                    e
                ),
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

    fn extract_typescript_imports(
        file: &File,
        resolver: &Resolver,
        project_root: &Path,
    ) -> Result<Vec<File>, Box<dyn std::error::Error>> {
        let source_code = std::fs::read_to_string(&file.path)?;
        let allocator = oxc_allocator::Allocator::default();
        let source_type = match file.path.extension().and_then(|s| s.to_str()) {
            Some("tsx") => oxc_span::SourceType::tsx(),
            Some("ts") => oxc_span::SourceType::ts(),
            Some("jsx") => oxc_span::SourceType::jsx(),
            Some("mjs") => oxc_span::SourceType::mjs(),
            Some("cjs") => oxc_span::SourceType::cjs(),
            _ => oxc_span::SourceType::unambiguous(),
        };

        let ParserReturn {
            program, errors, ..
        } = oxc_parser::Parser::new(&allocator, &source_code, source_type).parse();
        for e in &errors {
            eprintln!("Parse error in {}: {e}", file.path.display());
        }

        let mut visitor = ImportVisitor::new(&file.path, resolver, project_root);
        visitor.visit_program(&program);
        Ok(visitor.imports)
    }
}

struct ImportVisitor<'a> {
    imports: Vec<File>,
    current_file_dir: PathBuf,
    resolver: &'a Resolver,
    project_root: &'a Path,
}

impl<'a> ImportVisitor<'a> {
    fn new(current_file_path: &PathBuf, resolver: &'a Resolver, project_root: &'a Path) -> Self {
        Self {
            imports: Vec::new(),
            current_file_dir: current_file_path
                .parent()
                .unwrap_or(Path::new(""))
                .to_path_buf(),
            resolver,
            project_root,
        }
    }

    fn add_import(&mut self, specifier: &str) {
        println!("Found import: {}", specifier);
        // On délègue TOUT au résolveur (gère alias TS, baseUrl, exports, extensions, index.*)
        // Le contexte doit être un dossier absolu
        let context = if specifier.starts_with('.') {
            // imports relatifs => depuis le dossier du fichier courant
            self.current_file_dir.clone()
        } else {
            // alias/paquets => on résout depuis la racine du projet
            self.project_root.to_path_buf()
        };

        println!(
            "Resolving import '{}' from context '{}'",
            specifier,
            context.display()
        );

        if let Ok(resolution) = self.resolver.resolve(context, specifier) {
            println!(
                "Resolved import '{}' to '{}'",
                specifier,
                resolution.full_path().display()
            );
            let path = resolution.full_path().to_path_buf();
            self.imports.push(File::new(path));
        } else {
            println!("Failed to resolve import '{}'", specifier);
        }
    }
}

impl<'a> Visit<'a> for ImportVisitor<'a> {
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
