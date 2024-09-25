use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::workspace_lock::WorkspaceLock;

// but file name is package.toml
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceConfig {
    pub package: Option<Package>,
    pub dependencies: Option<HashMap<String, Dependency>>,
    #[serde(rename = "dev-dependencies")]
    pub dev_dependencies: Option<HashMap<String, Dependency>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed {
        version: Option<String>,
        github: Option<GithubDependency>,
        path: Option<String>,
    },
}

impl Dependency {
    pub fn get_version(&self) -> Option<&String> {
        match self {
            Dependency::Simple(version) => Some(version),
            Dependency::Detailed { version, .. } => version.as_ref(),
        }
    }

    pub fn get_github(&self) -> Option<&GithubDependency> {
        match self {
            Dependency::Simple(_) => None,
            Dependency::Detailed { github, .. } => github.as_ref(),
        }
    }

    pub fn try_merge_lock_dependency(&mut self, lock_dep: &Dependency) {
        match (self, lock_dep) {
            (
                Dependency::Detailed {
                    version: _,
                    github: self_github,
                    path: _,
                },
                Dependency::Detailed {
                    version: _,
                    github: lock_github,
                    path: _,
                },
            ) => {
                if let Some(lock_github) = lock_github {
                    if let Some(self_github) = self_github {
                        if self_github.tag.is_none() {
                            self_github.tag = lock_github.tag.clone();
                        }
                        if self_github.branch.is_none() {
                            self_github.branch = lock_github.branch.clone();
                        }
                        if self_github.hash.is_none() {
                            self_github.hash = lock_github.hash.clone();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct GithubDependency {
    pub url: String,
    pub tag: Option<String>,
    pub branch: Option<String>,
    pub hash: Option<String>,
}

impl WorkspaceConfig {
    pub fn new() -> Self {
        Self {
            package: None,
            dependencies: None,
            dev_dependencies: None,
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

    pub fn to_toml_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        let toml_str = self
            .to_toml_str()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(file_path, toml_str)
    }

    pub fn try_merge_lock_file(&mut self, lock_file_path: &str) {
        let lock_file = WorkspaceLock::parse_toml_file(lock_file_path);
        if let Ok(lock_file) = lock_file {
            if let Some(dependencies) = lock_file.dependencies {
                if let Some(self_dependencies) = &mut self.dependencies {
                    for (name, dep) in self_dependencies {
                        if let Some(lock_dep) = dependencies.get(name) {
                            dep.try_merge_lock_dependency(lock_dep);
                        }
                    }
                }

                if let Some(self_dev_dependencies) = &mut self.dev_dependencies {
                    for (name, dep) in self_dev_dependencies {
                        if let Some(lock_dep) = dependencies.get(name) {
                            dep.try_merge_lock_dependency(lock_dep);
                        }
                    }
                }
            }
        }
    }

    pub fn add_dependency(&mut self, name: String, dep: Dependency) {
        if let Some(deps) = &mut self.dependencies {
            deps.remove(&name);
            deps.insert(name, dep);
        } else {
            let mut deps = HashMap::new();
            deps.insert(name, dep);
            self.dependencies = Some(deps);
        }
    }

    pub fn add_dev_dependency(&mut self, name: String, dep: Dependency) {
        if let Some(deps) = &mut self.dev_dependencies {
            deps.remove(&name);
            deps.insert(name, dep);
        } else {
            let mut deps = HashMap::new();
            deps.insert(name, dep);
            self.dev_dependencies = Some(deps);
        }
    }

    pub fn remove_dependency(&mut self, name: &str) {
        if let Some(deps) = &mut self.dependencies {
            deps.remove(name);
        }
    }

    pub fn remove_dev_dependency(&mut self, name: &str) {
        if let Some(deps) = &mut self.dev_dependencies {
            deps.remove(name);
        }
    }

    pub fn get_dependency(&self, name: &str) -> Option<&Dependency> {
        self.dependencies.as_ref()?.get(name)
    }

    pub fn get_dev_dependency(&self, name: &str) -> Option<&Dependency> {
        self.dev_dependencies.as_ref()?.get(name)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dependencies_serialization() {
        let mut dependencies = HashMap::new();
        dependencies.insert("lua".to_string(), Dependency::Simple("5.4.7".to_string()));
        dependencies.insert(
            "ls_Framework".to_string(),
            Dependency::Detailed {
                version: Some("0.5.1".to_string()),
                github: Some(GithubDependency {
                    url: "https://github.com/CppCXY/LanguageServer.Framework.git".to_string(),
                    tag: None,
                    branch: None,
                    hash: None,
                }),
                path: None,
            },
        );

        let config = WorkspaceConfig {
            package: None,
            dependencies: Some(dependencies),
            dev_dependencies: None,
        };

        let serialized = config.to_toml_str().unwrap();
        println!("{}", serialized);
    }

    #[test]
    fn test_dependencies_deserialization() {
        let toml_str = r#"
            [dependencies]
            lua = "5.4.7"
            luamake = { version = "0.5.1" }

            [dev-dependencies]
            test-lib = { path = "../test-lib" }
            "#;

        let config: WorkspaceConfig = WorkspaceConfig::parse_toml_str(toml_str).unwrap();

        let dependencies = config.dependencies.unwrap();

        assert_eq!(
            dependencies.get("lua"),
            Some(&Dependency::Simple("5.4.7".to_string()))
        );

        assert_eq!(
            dependencies.get("luamake"),
            Some(&Dependency::Detailed {
                version: Some("0.5.1".to_string()),
                github: None,
                path: None,
            })
        );

        assert_eq!(
            config.dev_dependencies.unwrap().get("test-lib"),
            Some(&Dependency::Detailed {
                version: None,
                github: None,
                path: Some("../test-lib".to_string()),
            })
        );
    }

    #[test]
    fn test_workspace_config_serialization() {
        let package = Package {
            name: Some("example".to_string()),
            version: Some("0.1.0".to_string()),
            path: None,
        };

        let config = WorkspaceConfig {
            package: Some(package),
            dependencies: None,
            dev_dependencies: None,
        };

        let serialized = config.to_toml_str().unwrap();
        println!("{}", serialized);
    }

    #[test]
    fn test_workspace_config_deserialization() {
        let toml_str = r#"
            [package]
            name = "example"
            version = "0.1.0"
            "#;

        let config: WorkspaceConfig = WorkspaceConfig::parse_toml_str(toml_str).unwrap();

        let package = config.package.unwrap();
        assert_eq!(package.name, Some("example".to_string()));
        assert_eq!(package.version, Some("0.1.0".to_string()));
    }
}
