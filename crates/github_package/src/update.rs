use std::path::Path;

use git2::Error;
use lua_workspace_config::workspace_config::GithubDependency;

pub fn update_to_special_version(
    github_config: &GithubDependency,
    repo_path: &Path,
) -> Result<(), Error> {
    let repo = git2::Repository::open(repo_path)?;

    let head = repo.head()?;

    if let Some(branch) = &github_config.branch {
        if head.shorthand() != Some(branch) {
            let (object, reference) = repo.revparse_ext(&branch)?;
            repo.checkout_tree(&object, None)?;
            if let Some(reference) = reference {
                repo.set_head(&reference.name().unwrap())?;
            } else {
                repo.set_head_detached(object.id())?;
            }
        }
    }

    for i in 0..1 {
        let obj = if let Some(hash) = &github_config.hash {
            repo.revparse_single(&hash)?
        } else if let Some(tag) = &github_config.tag {
            repo.revparse_single(&tag)?
        } else {
            return Err(Error::from_str("No hash or tag provided"));
        };

        let head_commit = repo.head()?.peel_to_commit()?;
        if head_commit.id() != obj.id() {
            match repo.reset(&obj, git2::ResetType::Hard, None) {
                Ok(_) => {}
                Err(_) => {
                    if i == 0 {
                        let branch = head.shorthand().unwrap();

                        repo.find_remote("origin")?.fetch(&[branch], None, None)?;
                        let (object, reference) = repo.revparse_ext(&branch)?;
                        repo.checkout_tree(&object, None)?;
                        if let Some(reference) = reference {
                            repo.set_head(&reference.name().unwrap())?;
                        } else {
                            repo.set_head_detached(object.id())?;
                        }
                        continue;
                    } else {
                        return Err(Error::from_str("Failed to reset to hash"));
                    }
                }
            }
            break;
        }
    }

    Ok(())
}
