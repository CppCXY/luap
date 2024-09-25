use std::path::Path;

use lua_workspace_config::workspace_config::{Dependency, WorkspaceConfig};

use super::find_repo_path;


pub(crate) fn remove_package(package_name: &str) {
    let path = Path::new("package.toml");

    if !path.exists() {
        eprintln!("package.toml not found");
        return;
    }

    let mut config = WorkspaceConfig::parse_toml_file(path.to_str().unwrap()).unwrap();
    if let Some(deps) = &config.dependencies {
        if let Some(dep) = deps.get(package_name) {
            match dep {
                Dependency::Detailed { version, path, .. } => {
                    let repo_path = find_repo_path(package_name, version.clone(), path.clone());
                    let repo = repo_path.to_str().unwrap();
                    if Path::new(repo).exists() {
                        std::fs::remove_dir_all(repo).unwrap_or_else(|err| {
                            eprintln!("Failed to remove directory {}: {}", repo, err);
                        });
                    }
                }
                _ => {}
            }
            config.remove_dependency(package_name);
        }
    }
}

pub(crate) fn remove_dev_package(package_name: &str) {
    let path = Path::new("package.toml");

    if !path.exists() {
        eprintln!("package.toml not found");
        return;
    }

    let mut config = WorkspaceConfig::parse_toml_file(path.to_str().unwrap()).unwrap();
    if let Some(deps) = &config.dev_dependencies {
        if let Some(dep) = deps.get(package_name) {
            match dep {
                Dependency::Detailed { version, path, .. } => {
                    let repo_path = find_repo_path(package_name, version.clone(), path.clone());
                    let repo = repo_path.to_str().unwrap();
                    if Path::new(repo).exists() {
                        std::fs::remove_dir_all(repo).unwrap_or_else(|err| {
                            eprintln!("Failed to remove directory {}: {}", repo, err);
                        });
                    }
                }
                _ => {}
            }
            config.remove_dev_dependency(package_name);
        }
    }
}