use std::path::{Path, PathBuf};

use git2::{BranchType, Repository};
use crate::error::{MitosError, MitosResult};

#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub name: String,
    pub path: PathBuf,
    pub branch: Option<String>,
    pub locked: bool,
}

fn open_repo() -> MitosResult<Repository> {
    let repo = Repository::discover(".")?;
    Ok(repo)
}

pub fn list_worktrees() -> MitosResult<Vec<WorktreeInfo>> {
    let repo = open_repo()?;
    let mut infos: Vec<WorktreeInfo> = Vec::new();

    // include main worktree
    if let Some(workdir) = repo.workdir() {
        let name = workdir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        let branch = repo.head().ok().and_then(|h| h.shorthand().map(|s| s.to_string()));
        infos.push(WorktreeInfo { name, path: workdir.to_path_buf(), branch, locked: false });
    }

    // iterate additional worktrees via libgit2
    // Wrap in Ok() to handle potential errors from repo.worktrees()
    if let Ok(names) = repo.worktrees() {
        for name_result in names.iter() {
            // Skip invalid names instead of erroring
            if let Some(name) = name_result {
                // Skip if we can't find the worktree
                if let Ok(wt) = repo.find_worktree(&name) {
                    let path = wt.path().to_path_buf();
                    let branch = Repository::open(&path)
                        .ok()
                        .and_then(|r| r.head().ok().and_then(|h| h.shorthand().map(|s| s.to_string())));
                    let locked = matches!(wt.is_locked(), Ok(git2::WorktreeLockStatus::Locked(_)));
                    let wt_name = wt.name().unwrap_or(&name).to_string();
                    infos.push(WorktreeInfo { name: wt_name, path, branch, locked });
                }
            }
        }
    }

    Ok(infos)
}

fn ensure_local_branch(repo: &Repository, branch: &str) -> MitosResult<()> {
    match repo.find_branch(branch, BranchType::Local) {
        Ok(_) => Ok(()),
        Err(_) => {
            let head = repo.head()?;
            let target = head.peel_to_commit()?;
            repo.branch(branch, &target, false)?;
            Ok(())
        }
    }
}

pub fn create_worktree(branch: &str, path: Option<&Path>) -> MitosResult<PathBuf> {
    let repo = open_repo()?;
    ensure_local_branch(&repo, branch)?;

    let base = repo.workdir().ok_or_else(|| MitosError::GitError("Bare repository is not supported for worktrees".to_string()))?;
    let default_path = base.join(branch.replace('/', "-"));
    let target = path.map(|p| if p.is_relative() { base.join(p) } else { p.to_path_buf() }).unwrap_or(default_path);

    let name = target
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| MitosError::GitError("Invalid target path".to_string()))?
        .to_string();

    let mut opts = git2::WorktreeAddOptions::new();
    let branch_obj = repo.find_branch(branch, BranchType::Local)?;
    let branch_ref = branch_obj.into_reference();
    opts.reference(Some(&branch_ref));

    let _wt = repo.worktree(&name, &target, Some(&mut opts))?;
    Ok(target)
}

pub fn delete_worktree(name_or_path: &str, force: bool) -> MitosResult<()> {
    let repo = open_repo()?;
    let infos = list_worktrees()?;

    // Find the worktree info by name or path
    let worktree_info = if let Some(info) = infos.iter().find(|w| w.name == name_or_path) {
        info
    } else {
        let input_path = Path::new(name_or_path);
        let abs_path = if input_path.is_absolute() {
            input_path.to_path_buf()
        } else {
            std::env::current_dir()?.join(input_path)
        };
        
        infos.iter()
            .find(|w| w.path == abs_path || w.path.ends_with(name_or_path))
            .ok_or_else(|| MitosError::GitError(format!("worktree not found: {}", name_or_path)))?
    };

    // Skip deletion of main worktree
    if worktree_info.name == "mitos" || worktree_info.path == repo.workdir().unwrap() {
        return Err(MitosError::GitError("Cannot delete main worktree".to_string()));
    }

    // Store the path before deletion
    let worktree_path = worktree_info.path.clone();

    // Use the actual worktree name from the info
    let prune_result = repo.find_worktree(&worktree_info.name).and_then(|wt| {
        let mut prune_opts = git2::WorktreePruneOptions::new();
        prune_opts.valid(true);
        prune_opts.locked(!force);
        prune_opts.working_tree(true); // This should remove the working tree
        wt.prune(Some(&mut prune_opts))
    });

    // Always try to remove the directory, regardless of prune result
    if worktree_path.exists() {
        std::fs::remove_dir_all(&worktree_path)
            .map_err(|e| MitosError::GitError(format!("Failed to remove worktree directory: {}", e)))?;
    }

    // Clean up git metadata if prune failed
    if prune_result.is_err() {
        let git_worktree_dir = repo.path().join("worktrees").join(&worktree_info.name);
        if git_worktree_dir.exists() {
            std::fs::remove_dir_all(&git_worktree_dir)
                .map_err(|e| MitosError::GitError(format!("Failed to clean git metadata: {}", e)))?;
        }
    }

    Ok(())
}
