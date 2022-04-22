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
           