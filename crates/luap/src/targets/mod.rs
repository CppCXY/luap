use std::path::{Path, PathBuf};

pub mod install;
pub mod add;
pub mod check;
pub mod init;
pub mod remove;
pub mod update;

pub(crate) fn find_library_path(base_path: &Path, path: Option<String>) -> PathBuf {
    if let Some(path) = path {
        base_path.join(path)
    } else {
        let library_path = base_path.join("library");
        if library_path.exists() {
            return library_path;
        }
        let lib_path = base_path.join("lib");
        if lib_path.exists() {
            return lib_path;
        }
        return base_path.to_path_buf();
    }
}

pub(crate) fn find_repo_path(name: &str, version: Option<String>, path: Option<String>) -> PathBuf {
    if let Some(path) = path {
        return PathBuf::from(path);
    }

    let mut base_path = PathBuf::from("lua_modules");
    base_path.push(name);
    if let Some(version) = version {
        base_path.push(version);
    }
    
    base_path
}