use std::{error::Error, path::Path};

use lua_workspace_config::workspace_config::GithubDependency;

use crate::resolve_github_url;

pub fn get_dep_from_repo(repo_path: &Path, url: &str) -> Result<GithubDependency, Box<dyn Error>> {
    let repo = git2::Repository::open(repo_path)?;

    // let head = repo.head()?;

    let mut dep = GithubDependency {
        url: url.to_string(),
        branch: None,
        tag: None,
        hash: None,
    };

    let remote = repo.find_remote("origin")?;
    let url = remote.url().unwrap();
    dep.url = resolve_github_url(url);

    let head_commit = repo.head()?.peel_to_commit()?;
    dep.hash = Some(head_commit.id().to_string());

    Ok(dep)
}