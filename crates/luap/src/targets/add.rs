use std::path::Path;

use lua_workspace_config::workspace_config::{Dependency, GithubDependency, WorkspaceConfig};

use super::{init::init_package, install::install_package};

pub fn add_package(package_name: &str, github_repo: &str, branch: Option<String>, tag: Option<String>, hash: Option<String>) {
    match inner_add_package(package_name, github_repo, branch, tag, hash) {
        Ok(_) => {
            eprintln!("Add package success");
        }
        Err(e) => {
            eprintln!("Failed to add package: {}", e);
        }
    }
}

pub fn add_dev_package(package_name: &str, github_repo: &str, branch: Option<String>, tag: Option<String>, hash: Option<String>) {
    match inner_add_package(package_name, github_repo, branch, tag, hash) {
        Ok(_) => {
            eprintln!("Add dev dependency success");
        }
        Err(e) => {
            eprintln!("Failed to add dev dependency: {}", e);
        }
    }
}

fn inner_add_package(package_name: &str, github_repo: &str, branch: Option<String>, tag: Option<String>, hash: Option<String>) -> Result<(), std::io::Error> {
    let package_toml_path = Path::new("package.toml");
    if !package_toml_path.exists() {
        init_package();
        return Ok(());
    }

    let mut config = WorkspaceConfig::parse_toml_file(package_toml_path.to_str().unwrap())?;
    let dep = Dependency::Detailed {
        version: None,
        github: Some(GithubDependency {
            url: github_repo.to_string(),
            branch,
            tag,
            hash,
        }),
        path: None,
    };

    config.add_dependency(package_name.to_owned(), dep);
    config.to_toml_file(package_toml_path.to_str().unwrap())?;
    install_package(false);
    Ok(())
}
