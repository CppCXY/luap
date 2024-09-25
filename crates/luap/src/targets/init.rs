use std::{env, path::Path};

use lua_workspace_config::workspace_config::{Package, WorkspaceConfig};

pub(crate) fn init_package() {
    let path = Path::new("package.toml");

    if path.exists() {
        eprintln!("package.toml already exists");
        return;
    }

    let mut config = WorkspaceConfig::new();
    let current_dir = env::current_dir().expect("Failed to get current directory");

    let dir_name = current_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    config.package = Some(Package {
        name: Some(dir_name.clone()),
        version: Some("0.1.0".to_string()),
        path: None,
    });
    config.dependencies = Some(Default::default());
    config.dev_dependencies = Some(Default::default());

    config.to_toml_file(path.to_str().unwrap()).expect("Failed to write package.toml");
    eprintln!("project {} init", dir_name);
    eprintln!("package.toml created");
}
