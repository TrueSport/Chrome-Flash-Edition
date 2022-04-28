pub mod error;
pub mod modes;

use std::path::{Path, PathBuf};
use scribe::Workspace;
use crate::view::{Colors, StatusLineData, Style};
use git2::{self, Repository, Status};

fn path_as_title(path: &Path) -> String {
    format!(" {}", path.to_string_lossy())
}

fn current_buffer_status_line_data(workspace: &mut Workspace) -> StatusLineData {
    let modified = workspace.current_buffer().map(|b| b.modified()).unwrap_or(false);

    let (content, style) = workspace.current_buffer_path().map(|path| {
        // Determine buffer title styles based on its modification status.
        if modified {
            // Use an emboldened path with an asterisk.
            let mut title = path_as_title(path);
            title.push('*');

            (title, Style::Bold)
        } else {
            (path_as_title(path), Style::Default)
        }
    }).unwrap_or((String::new(), Style::Default));

    StatusLineData {
        content,
        style,
        colors: Colors::Focused,
    }
}

fn git_status_line_data(repo: &Option<Repository>, path: &Option<PathBuf>) -> StatusLineData {
    // Build a display value for the current buffer's git status.
    let mut content = String::new();
    if let Some(ref repo) = *repo {
        if let Some(ref path) = *path {
            if let Some(repo_path) = repo.workdir() {
                if let Ok(relative_path) = path.strip_prefix(repo_path) {
                    if let Ok(status) = repo.status_file(relative_path) {
                        content = presentable_status(&status).to_string();
                    }
                }
            }
        }
    }

    StatusLineData {
        content,
        style: Style::Default,
        colors: Colors::Focused,
    }
}
fn presentable_status(status: &Status) -> &str {
    if status.contains(git2::Status::WT_NEW) {
        if status.contains(git2::Status::INDEX_NEW) {
            // Parts of the file are staged as new in the index.
            "[partially staged]"
        } else {
            // The file has never been added to the repository.
            "[untracked]"
        }
    } else if status.contains(git2::Status::INDEX_NEW) {
        // The complete file is staged as new in the index.
        "[staged]"
    } else if status.contains(git2::Status::WT_MODIFIED) {
        if status.contains(git2::Status::INDEX_MODIFIED) {
            // The file has both staged and unstaged modifications.
            "[partially staged]"
        } else {
            // The file has unstaged modifications.
            "[modified]"
        }
    } else if status.contains(git2::Status::INDEX_MODIFIED) {
        // The file has staged modifications.
        "[staged]"
    } else {
        // The file is tracked, but has no modifications.
        "[ok]"
    }
}

#[cfg(test)]
mod tests {
    use git2;
    use super::presentable_statu