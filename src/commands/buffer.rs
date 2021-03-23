use crate::errors::*;
use crate::commands::{self, Result};
use std::mem;
use crate::input::Key;
use crate::util;
use crate::util::token::{Direction, adjacent_token_position};
use crate::models::application::{Application, ClipboardContent, Mode};
use crate::models::application::modes::ConfirmMode;
use scribe::buffer::{Buffer, Position, Range};

pub fn save(app: &mut Application) -> Result {
    remove_trailing_whitespace(app)?;
    ensure_trailing_newline(app)?;

    // Slight duplication here, but we need to check for a buffer path without
    // borrowing the buffer for the full scope of this save command. That will
    // allow us to hand the application object to the switch_to_path_mode
    // command, if necessary.
    let path_set = app
        .workspace
        .current_buffer()
        .ok_or(BUFFER_MISSING)?
        .path.is_some();

    if path_set {
        app.workspace
            .current_buffer()
            .ok_or(BUFFER_MISSING)?
            .save()
            .chain_err(|| "Unable to save buffer")
    } else {
        commands::application::switch_to_path_mode(app)?;
        if let Mode::Path(ref mut mode) = app.mode {
            mode.save_on_accept = true;
        }

        Ok(())
    }
}

pub fn reload(app: &mut Application) -> Result {
    app.workspace.current_buffer().ok_or(BUFFER_MISSING)?.reload().chain_err(|| {
        "Unable to reload buffer."
    })
}

pub fn delete(app: &mut Application) -> Result {
    app.workspace.current_buffer().ok_or(BUFFER_MISSING)?.delete();
    commands::view::scroll_to_cursor(app)?;

    Ok(())
}

pub fn delete_token(app: &mut Application) -> Result {
    let mut subsequent_token_on_line = false;

    if let Some(buffer) = app.workspace.current_buffer() {
        if let Some(position) = adjacent_token_position(buffer, false, Direction::Forward) {
            if position.line == buffer.cursor.line {
                subsequent_token_on_line = true;
            }
        }
    } else {
        bail!(BUFFER_MISSING);
    }

    if subsequent_token_on_line {
        commands::application::switch_to_select_mode(app)?;
        commands::cursor::move_to_start_of_next_token(app)?;
        commands::selection::copy_and_delete(app)?;
        commands::application::switch_to_normal_mode(app)?;
        commands::view::scroll_to_cursor(app)?;
    } else {
        commands::buffer::delete_rest_of_line(app)?;
    }

    Ok(())
}

pub fn delete_current_line(app: &mut Application) -> Result {
    commands::application::switch_to_select_line_mode(app)?;
    commands::selection::copy_and_delete(app)?;
    commands::application::switch_to_normal_mode(app)?;
    commands::view::scroll_to_cursor(app)?;

    Ok(())
}

pub fn copy_current_line(app: &mut Application) -> Result {
    commands::application::switch_to_select_line_mode(app)?;
    commands::selection::copy(app)?;
    commands::application::switch_to_normal_mode(app)?;
    commands::view::scroll_to_cursor(app)?;

    Ok(())
}

pub fn merge_next_line(app: &mut Application) -> Result {
    let buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;
