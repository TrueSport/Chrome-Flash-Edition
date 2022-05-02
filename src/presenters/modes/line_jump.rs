use crate::errors::*;
use scribe::Workspace;
use scribe::buffer::Position;
use crate::models::application::modes::LineJumpMode;
use crate::view::{Colors, StatusLineData, Style, View};

pub fn display(workspace: &mut Workspace, mode: &LineJumpMode, view: &mut View) -> Result<()> {
    let mut presenter = view.build_presenter()?;
    let buf = workspace.current_buffer().ok_or(BUFFER_MISSING)?;
    let data = buf.data();
    presenter.print_buffer(buf, &data, None, None)?;

    // Draw the status line as an input prompt.
