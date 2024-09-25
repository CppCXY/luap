use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};
use lua_workspace_config::workspace_config::GithubDependency;
use std::path::Path;

use crate::{find_id_rsa, resolve_github_url};

pub fn clone_and_init_submodules(
    github_config: &GithubDependency,
    to_path: &Path,
) -> Result<(), git2::Error> {
    // Set up callbacks for authentication
    let mut callbacks = RemoteCallbacks::new();
    let ssh_id_rsa = find_id_rsa();

    if let Some(ssh_id_rsa) = ssh_id_rsa {
        callbacks.credentials(move |_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                Path::new(&ssh_id_rsa),
                None,
            )
        });
    } else {
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key_from_agent(username_from_url.unwrap())
        });
    }

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);

    if let Some(branch) = &github_config.branch {
        builder.branch(&branch);
    }

    let repo_url = &resolve_github_url(&github_config.url);

    let repo = builder.clone(repo_url, to_path)?;

    if let Some(hash) = &github_config.hash {
        let obj = repo.revparse_single(&hash)?;
        repo.reset(&obj, git2::ResetType::Hard, None)?;
    } else if let Some(tag) = &github_config.tag {
        let obj = repo.revparse_single(&tag)?;
        repo.reset(&obj, git2::ResetType::Hard, None)?;
    }

    // Initialize submodules
    for mut submodule in repo.submodules()? {
        submodule.update(true, None)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_clone_and_init_submodules() {
        let repo_url = "https://github.com/LuaLS/lua-language-server-rust.git";
        let path = PathBuf::from("test_repo");

        // Clean up before test
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }

        let github_config = GithubDependency {
            url: repo_url.to_string(),
            tag: None,
            branch: None,
            hash: None,
        }; 
        // Clone and initialize submodules
        clone_and_init_submodules(&github_config, &path).unwrap();

        // Verify the repository was cloned
        assert!(path.join(".git").exists());

        // Clean up after test
        // fs::remove_dir_all(&path).unwrap();
    }

    #[test]
    fn test_find_id_rsa() {
        let ssh_id_rsa = find_id_rsa();
        assert!(ssh_id_rsa.is_some());
    }
}
