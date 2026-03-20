use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CommandRegistry {
    handlers: HashMap<String, String>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, name: impl Into<String>, handler: impl Into<String>) {
        self.handlers.insert(name.into(), handler.into());
    }

    pub fn resolve(&self, name: &str) -> Option<&str> {
        self.handlers.get(name).map(String::as_str)
    }

    pub fn names(&self) -> Vec<&str> {
        let mut names = self.handlers.keys().map(String::as_str).collect::<Vec<_>>();
        names.sort_unstable();
        names
    }

    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct CommandSpec {
    pub name: String,
    pub handler: String,
}

impl CommandSpec {
    pub fn new(name: impl Into<String>, handler: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            handler: handler.into(),
        }
    }
}
