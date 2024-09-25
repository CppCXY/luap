use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::workspace_config::Dependency;

// but file name is package.toml
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceLock {
    pub dependencies: Option<HashMap<String, Dependency>>,
}

impl WorkspaceLock {
    pub fn new() -> Self {
        Self {
            dependencies: None,
        }
    }

    pub fn parse_toml_str(toml: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml)
    }

    pub fn to_toml_str(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    pub fn parse_toml_file(file_path: &str) -> Result<Self, std::io::Error> {
        let toml_str = std::fs::read_to_string(file_path)?;
        Self::parse_toml_str(&toml_str)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn write_toml_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        let toml_str = self
            .to_toml_str()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(file_path, toml_str)
    }

    pub fn add_dependency(&mut self, name: String, dep: Dependency) {
        if let Some(deps) = &mut self.dependencies {
            deps.insert(name, dep);
        } else {
            let mut deps = HashMap::new();
            deps.insert(name, dep);
            self.dependencies = Some(deps);
        }
    }
}