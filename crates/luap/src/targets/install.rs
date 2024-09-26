use indicatif::{ProgressBar, ProgressStyle};
use lua_workspace_config::workspace_config::{Dependency, GithubDependency, WorkspaceConfig};
use std::{path::Path, thread};

use crate::lock_file::gen_lock_file;

use super::{find_library_path, find_repo_path};

pub fn install_package(dump_library: bool) {
    let base_path = std::env::current_dir().unwrap();
    let base_path = Path::new(&base_path);
    let mut results: Vec<String> = Vec::new();
    try_install_package(base_path, &mut results);
    match gen_lock_file(base_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to generate lock file: {}", e);
        }
    }

    if dump_library {
        for path in results {
            println!("{}", path);
        }
    } else {
        eprintln!("Install package success");
    }
}

fn try_install_package(base_path: &Path, results: &mut Vec<String>) {
    let package_path = base_path.join("package.toml");
    if !package_path.exists() {
        return;
    }

    let config = WorkspaceConfig::parse_toml_file(&package_path.to_str().unwrap());
    if config.is_err() {
        eprintln!("Failed to parse package.toml: {:?}", config.err().unwrap());
        return;
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

    for (name, dep) in config.dependencies.unwrap() {
        check_and_install_package(&name, &dep, results, false);
    }

    for (name, dep) in config.dev_dependencies.unwrap_or_default() {
        check_and_install_package(&name, &dep, results, true);
    }
}

fn check_and_install_package(name: &str, dep: &Dependency, results: &mut Vec<String>, dev: bool) {
    let version = dep.get_version();
    let github = dep.get_github_dependency();
    let path = dep.get_path();

    if let Some(version) = version {
        eprintln!(
            "Installing dependency package: {}@{}, but current not support version",
            name, version
        );
    }

    let to_path = find_repo_path(name, dep.get_version(), path.clone());
    check_and_install_github_package(name, &github, to_path.as_path());
    if !dev {
        try_install_package(&to_path, results);
    }
    let library_path = find_library_path(to_path.as_path(), None);
    results.push(library_path.to_str().unwrap().to_string());
}

fn check_and_install_github_package(name: &str, github_config: &GithubDependency, to_path: &Path) {
    if to_path.exists() {
        match github_package::check_github_repo_version(github_config, to_path) {
            Ok(true) => {
                return;
            }
            Ok(false) => {
                eprintln!(
                    "Updating dependency package: {} from github to {}",
                    name,
                    to_path.to_str().unwrap()
                );
                let _ = github_package::update_to_special_version(github_config, to_path);
            }
            Err(e) => {
                eprintln!(
                    "Failed to check version of {}, error: {}",
                    to_path.to_str().unwrap(),
                    e
                );
            }
        }
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner} {msg}")
                .unwrap()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
        );

        pb.set_draw_target(indicatif::ProgressDrawTarget::stderr());
        pb.set_message(format!(
            "Cloning dependency package: {} from github to {}",
            name,
            to_path.to_str().unwrap()
        ));

        let new_github_config = github_config.clone();
        let new_to_path = to_path.to_path_buf();
        let handle = thread::spawn(move || {
            let _ = github_package::clone_and_init_submodules(
                &new_github_config,
                new_to_path.as_path(),
            );
        });

        while !handle.is_finished() {
            pb.tick();
            thread::sleep(std::time::Duration::from_millis(100));
        }
        pb.finish_with_message(format!("Install dependency package: {}!", name));
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs, io::Write, path::PathBuf};

    use super::*;

    fn create_temp_package_toml(dir: &Path, content: &str) {
        let package_path = dir.join("package.toml");
        let mut file = fs::File::create(package_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    fn tempdir() -> Option<PathBuf> {
        std::env::temp_dir().canonicalize().ok()
    }

    #[test]
    fn test_install_package_no_package_toml() {
        let dir = tempdir().unwrap();
        env::set_current_dir(&dir).unwrap();

        install_package(false);

        // Since there's no package.toml, results should be empty
        // We can check the output manually or redirect stdout to capture the output
    }

    #[test]
    fn test_install_package_with_package_toml() {
        let dir = tempdir().unwrap();
        let dir = Path::new(&dir);
        env::set_current_dir(&dir).unwrap();

        let package_toml_content = r#"
            [package]
            path = "src/main.rs"
            "#;
        create_temp_package_toml(dir, package_toml_content);

        install_package(false);

        // Check if the path "src/main.rs" is printed
        // We can check the output manually or redirect stdout to capture the output
    }

    #[test]
    fn test_check_and_install_package_with_dependencies() {
        let dir = tempdir().unwrap();
        let dir = Path::new(&dir);
        env::set_current_dir(&dir).unwrap();

        let package_toml_content = r#"
            [package]
            path = "src/main.rs"

            [dependencies]
            dep1 = "0.1"
            dep2 = { version = "0.2", github = { url = "https://github.com/example/repo" } }
            "#;
        create_temp_package_toml(dir, package_toml_content);

        let mut results: Vec<String> = Vec::new();
        try_install_package(dir, &mut results);

        // Check if the paths are correctly added to results
        // We can check the output manually or redirect stdout to capture the output
    }

    #[test]
    fn test_check_and_install_package_with_dev_dependencies() {
        let dir = tempdir().unwrap();
        let dir = Path::new(&dir);
        env::set_current_dir(&dir).unwrap();

        let package_toml_content = r#"
            [package]
            path = "main"

            [dependencies]
            ls.Framework = { 
                path = "3rd/ls.Framework"
                github = {
                    url = "https://github.com/CppCXY/LanguageServer.Framework.git"
                }
            }
            "#;
        create_temp_package_toml(dir, package_toml_content);

        let mut results: Vec<String> = Vec::new();
        try_install_package(dir, &mut results);

        // Check if the paths are correctly added to results
        // We can check the output manually or redirect stdout to capture the output
    }
}
