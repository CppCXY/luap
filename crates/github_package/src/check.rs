use std::path::Path;

use git2::{Repository, Error};
use lua_workspace_config::workspace_config::GithubDependency;


pub fn check_github_repo_version(github_config: &GithubDependency, repo_path: &Path) -> Result<bool, Error> {
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(_) => return Ok(false),
    };

    let head = repo.head()?;
    if let Some(branch) = &github_config.branch {
        if head.shorthand() != Some(branch) {
            return Ok(false);
        }
    }
    else {
        match head.shorthand() {
            Some("master") => (),
            Some("main") => (),
            _ => return Ok(false),
        }
    }

    let head_commit = head.peel_to_commit()?;
    if let Some(hash) = &github_config.hash {
        if head_commit.id().to_string() != *hash {
            return Ok(false);
        }
    }
    else if let Some(tag) = &github_config.tag {
        let tag_commit = repo.revparse_single(tag)?.peel_to_commit()?;
        if head_commit.id() != tag_commit.id() {
            return Ok(false);
        }
    }

    Ok(true)
}