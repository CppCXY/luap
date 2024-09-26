pub mod clone;
pub mod check;
pub mod update;
pub mod dep;

use dirs::home_dir;
pub use clone::clone_and_init_submodules;
pub use check::check_github_repo_version;
pub use update::*;


pub fn find_id_rsa() -> Option<String> {
    let home = home_dir()?;
    let ssh_id_rsa = home.join(".ssh/id_rsa");
    if ssh_id_rsa.exists() {
        Some(ssh_id_rsa.to_str().unwrap().to_string())
    } else {
        None
    }
}

pub fn resolve_github_url(url: &str) -> String {
    if url.starts_with("https://") || url.starts_with("git@github.com") {
        url.to_string()
    } else {
        format!("https://github.com/{}", url)
    }
}
