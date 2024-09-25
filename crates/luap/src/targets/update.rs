use std::path::Path;

use lua_workspace_config::workspace_config::WorkspaceConfig;

pub fn update_package(package_name: &str, branch: Option<String>, tag: Option<String>, hash: Option<String>) {
    let path = Path::new("package.toml");

    if !path.exists() {
        eprintln!("package.toml not found");
        return;
    }

    #[allow(unused_variables)]
    let mut config = WorkspaceConfig::parse_toml_file(path.to_str().unwrap()).unwrap();
    
    #[allow(unused_variables)]
    let dep = config.get_dependency(package_name);

    // if let Some(dep) = dep {
        
    // }
    
}

pub fn update_dev_package(package_name: &str, branch: Option<String>, tag: Option<String>, hash: Option<String>) {
    // update_package(package_name, branch, tag, hash, true);
}