use std::error::Error;
use std::path::Path;

use github_package::dep::get_dep_from_repo;
use lua_workspace_config::workspace_config::{Dependency, WorkspaceConfig};
use lua_workspace_config::workspace_lock::WorkspaceLock;

use crate::targets::find_repo_path;

pub(crate) fn gen_lock_file(base_path: &Path) -> Result<(), Box<dyn Error>> {
    let package_path = base_path.join("package.toml");
    if !package_path.exists() {
        return Ok(());
    }

    let config = WorkspaceConfig::parse_toml_file(&package_path.to_str().unwrap())?;
    let mut lock_file = WorkspaceLock::new();

    if let Some(deps) = &config.dependencies {
        for (name, dep) in deps {
            if let Dependency::Detailed { version, github, path } = dep {
                if let Some(github) = github {
                    let repo_path = find_repo_path(name, version.clone(), path.clone());
                    let github_dep = get_dep_from_repo(repo_path.as_path(), &github.url)?;
                    let new_dep = Dependency::Detailed {
                        version: version.clone(),
                        github: Some(github_dep),
                        path: path.clone(),
                    };
                    lock_file.add_dependency(name.to_string(), new_dep);
                    gen_lock_file(&repo_path)?;
                }
            }
        }
    }

    if let Some(dev_deps) = &config.dev_dependencies {
        for (name, dep) in dev_deps {
            if let Dependency::Detailed { version, github, path } = dep {
                if let Some(github) = github {
                    let repo_path = find_repo_path(name, version.clone(), path.clone());
                    let github_dep = get_dep_from_repo(repo_path.as_path(), &github.url)?;
                    let new_dep = Dependency::Detailed {
                        version: version.clone(),
                        github: Some(github_dep),
                        path: path.clone(),
                    };
                    lock_file.add_dependency(name.to_string(), new_dep);
                }
            }
        }
    }

    let lock_file_path = base_path.join("package.lock");
    lock_file.write_toml_file(&lock_file_path.to_str().unwrap())?;
    Ok(())
}
