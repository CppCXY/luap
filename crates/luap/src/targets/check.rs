use std::path::Path;

use github_package::check_github_repo_version;
use lua_workspace_config::workspace_config::{Dependency, WorkspaceConfig};

use super::{find_library_path, find_repo_path};

pub fn check_package(dump_library: bool) {
    let base_path = std::env::current_dir().unwrap();
    let base_path = Path::new(&base_path);
    let mut results: Vec<String> = Vec::new();
    match try_check_package(base_path, &mut results) {
        Ok(true) => {},
        Ok(false) => {
            eprintln!("Check package failed");
            std::process::exit(1);
        },
        Err(e) => {
            eprintln!("Failed to check package: {}", e);
            std::process::exit(1);
        }
    }

    if dump_library {
        for path in results {
            println!("{}", path);
        }
    } else {
        eprintln!("Check package success");
    }
}

fn try_check_package(base_path: &Path, results: &mut Vec<String>) -> Result<bool, std::io::Error> {
    let package_path = base_path.join("package.toml");
    if !package_path.exists() {
        return Ok(true);
    }

    let config = WorkspaceConfig::parse_toml_file(&package_path.to_str().unwrap());
    if config.is_err() {
        eprintln!("Failed to parse package.toml: {:?}", config.err().unwrap());
        return Ok(false);
    }
    let mut config = config.unwrap();
    let lock_file_path = base_path.join("package.lock");
    if lock_file_path.exists() {
        config.try_merge_lock_file(&lock_file_path.to_str().unwrap());
    }

    if let Some(package) = &config.package {
        let library_path = find_library_path(base_path, package.path.clone());
        results.push(library_path.to_str().unwrap().to_string());
    }

    let mut result = true;
    for (name, dep) in config.dependencies.unwrap() {
        result &= inner_check_package(&name, &dep, results, false)?;
    }

    for (name, dep) in config.dev_dependencies.unwrap_or_default() {
        result &= inner_check_package(&name, &dep, results, true)?;
    }

    Ok(result)
}

fn inner_check_package(
    name: &str,
    dep: &Dependency,
    results: &mut Vec<String>,
    dev: bool,
) -> Result<bool, std::io::Error> {
    match dep {
        Dependency::Detailed {
            version,
            github,
            path,
        } => {
            if let Some(github) = github {
                let repo_path = find_repo_path(name, version.clone(), path.clone());
                let library_path = find_library_path(&repo_path, path.clone());
                results.push(library_path.to_str().unwrap().to_string());

                let succ = check_github_repo_version(&github, &repo_path)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                if !succ {
                    eprintln!("Check package failed: {}", name);
                }

                if !dev {
                    try_check_package(&repo_path, results)?;
                }
                return Ok(succ);
            }
        }
        _ => {}
    }

    Ok(false)
}
