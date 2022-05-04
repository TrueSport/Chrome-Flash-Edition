use crate::errors::*;
use crate::models::application::modes::SelectLineMode;
use scribe::Workspace;
use crate::presenters::current_buffer_status_line_data;
use crate::view::{Colors, StatusLineData, Style, View};

pub fn display(workspace: &mut Workspace, mode: &SelectLineMode, view: &mut View) -> Result<()> {
    let mut presenter = view.build_presenter()?;
    let buffer_status = current_buffer_status_line_data(workspace);
    let buf = workspace.current_buffer().ok_or(BUFFER_MISSING)?;
    let selected_range = mode.to_range(&*buf.cursor);
    let data = buf.data();