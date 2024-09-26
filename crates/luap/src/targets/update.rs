use std::path::Path;

use github_package::{update_to_latest, update_to_special_version};
use lua_workspace_config::workspace_config::{GithubDependency, WorkspaceConfig};

use crate::lock_file::gen_lock_file;

use super::find_repo_path;

pub fn update_package(
    package_name: Option<String>,
    branch: Option<String>,
    tag: Option<String>,
    hash: Option<String>,
) {
    let path = Path::new("package.toml");

    if !path.exists() {
        eprintln!("package.toml not found");
        return;
    }

    let mut config = WorkspaceConfig::parse_toml_file(path.to_str().unwrap()).unwrap();

    if let Some(package_name) = package_name {
        update_one_package(&mut config, &package_name, branch, tag, hash);
    } else {
        update_all_package(&mut config);
    }
    gen_lock_file(path).unwrap_or_else(|err| {
        eprintln!("Failed to generate lock file: {}", err);
    });

    eprintln!("Update package success");
}

fn update_all_package(config: &mut WorkspaceConfig) {
    if let Some(dependencies) = &mut config.dependencies {
        let package_names: Vec<String> = dependencies.keys().cloned().collect();
        for name in package_names {
            update_one_package(config, &name, None, None, None);
        }
    }

    if let Some(dev_dependencies) = &config.dev_dependencies {
        let dev_package_names: Vec<String> = dev_dependencies.keys().cloned().collect();
        for name in dev_package_names {
            update_one_package(config, &name, None, None, None);
        }
    }
}

fn update_one_package(
    config: &mut WorkspaceConfig,
    package_name: &str,
    branch: Option<String>,
    tag: Option<String>,
    hash: Option<String>,
) {
    if branch.is_some() || tag.is_some() || hash.is_some() {
        update_one_package_to_special(config, package_name, branch, tag, hash);
    } else {
        update_one_package_to_latest(config, package_name);
    }
}

fn update_one_package_to_latest(config: &mut WorkspaceConfig, package_name: &str) {
    let dep = if let Some(dep) = config.get_dependency(package_name) {
        dep
    } else if let Some(dep) = config.get_dev_dependency(package_name) {
        dep
    } else {
        eprintln!("Package {} not found in dependencies or dev_dependencies", package_name);
        return;
    };
    let path = dep.get_path();
    let version = dep.get_version();
    let repo_path = find_repo_path(package_name, version.clone(), path.clone());
    match update_to_latest(repo_path.as_path()) {
        Ok(_) => {
            eprintln!("Update package {} to latest success", package_name);
        }
        Err(e) => {
            eprintln!("Failed to update package {}: {}", package_name, e);
        }
    }
}

fn update_one_package_to_special(
    config: &mut WorkspaceConfig,
    package_name: &str,
    branch: Option<String>,
    tag: Option<String>,
    hash: Option<String>,
) {
    let dep = if let Some(dep) = config.get_dependency(package_name) {
        dep
    } else if let Some(dep) = config.get_dev_dependency(package_name) {
        dep
    } else {
        eprintln!("Package {} not found in dependencies or dev_dependencies", package_name);
        return;
    };
    let path = dep.get_path();
    let version = dep.get_version();
    let repo_path = find_repo_path(package_name, version.clone(), path.clone());
    let repo = repo_path.as_path();
    if !repo.exists() {
        eprintln!("Repo path {} not found", repo.to_str().unwrap());
        return;
    }

    let github_dep = GithubDependency {
        url: dep.get_url(),
        branch,
        tag,
        hash,
    };

    match update_to_special_version(&github_dep, repo) {
        Ok(_) => {
            eprintln!("Update package {} to special version success", package_name);
        }
        Err(e) => {
            eprintln!("Failed to update package {}: {}", package_name, e);
        }
    }
}
