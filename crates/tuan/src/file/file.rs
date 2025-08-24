#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File {
    pub name: String,
}

impl File {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
