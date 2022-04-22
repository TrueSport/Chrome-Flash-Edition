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
    let 