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
    let current_line = buffer.cursor.line;
    let data = buffer.data();

    // Don't bother if there isn't a line below.
    data.lines().nth(current_line + 1).ok_or("No line below current line")?;

    // Join the two lines.
    let mut merged_lines: String = buffer.data()
                                         .lines()
                                         .enumerate()
                                         .skip(current_line)
                                         .take(2)
                                         .map(|(index, line)| {
                                             if index == current_line {
                                                 format!("{} ", line)
                                             } else {
                                                 line.trim_start().to_string()
                                             }
                                         })
                                         .collect();

    // Append a newline if there is a line below the next.
    if buffer.data().lines().nth(current_line + 2).is_some() {
        merged_lines.push('\n');
    }

    // Remove the two lines, move to the start of the line,
    // insert the merged lines, and position the cursor,
    // batched as a single operation.
    buffer.start_operation_group();
    let target_position = Position {
        line: current_line,
        offset: data.lines().nth(current_line).unwrap().len(),
    };
    buffer.delete_range(Range::new(Position {
                                       line: current_line,
                                       offset: 0,
                                   },
                                   Position {
                                       line: current_line + 2,
                                       offset: 0,
                                   }));
    buffer.cursor.move_to(Position {
        line: current_line,
        offset: 0,
    });
    buffer.insert(merged_lines);
    buffer.cursor.move_to(target_position);
    buffer.end_operation_group();

    Ok(())
}

pub fn close(app: &mut Application) -> Result {
    // Build confirmation check conditions.
    let (unmodified, empty) =
        if let Some(buf) = app.workspace.current_buffer() {
            (!buf.modified(), buf.data().is_empty())
        } else {
            bail!(BUFFER_MISSING);
        };
    let confirm_mode =
        if let Mode::Confirm(_) = app.mode {
            true
        } else {
            false
        };

    if unmodified || empty || confirm_mode {
        // Clean up view-related data for the buffer.
        app.view.forget_buffer(
            app.workspace.current_buffer().ok_or(BUFFER_MISSING)?
        )?;
        app.workspace.close_current_buffer();
    } else {
        // Display a confirmation prompt before closing a modified buffer.
        let confirm_mode = ConfirmMode::new(close);
        app.mode = Mode::Confirm(confirm_mode);
    }

    Ok(())
}

pub fn close_others(app: &mut Application) -> Result {
    // Get the current buffer's ID so we know what *not* to close.
    let id = app.workspace.current_buffer().map(|b| b.id).ok_or(BUFFER_MISSING)?;
    let mut modified_buffer = false;

    loop {
        // Try to advance to the next buffer. Handles two important states:
        //
        // 1. The initial state, where we haven't advanced beyond the
        //    the original/desired buffer.
        // 2. When a buffer that is being closed is positioned *after* the
        //    original buffer. Closing a buffer in this scenario selects the
        //    preceding buffer, which, without advancing, would be
        //    incorrectly interpreted as the completion of this process.
        if app.workspace.current_buffer().map(|b| b.id) == Some(id) {
            app.workspace.next_buffer();
        }

        // If we haven't yet looped back to the original buffer,
        // clean up view-related data and close the current buffer.
        if let Some(buf) = app.workspace.current_buffer() {
            if buf.id == id {
                // We've only got one buffer open; we're done.
                break;
            } else if buf.modified() && !buf.data().is_empty() {
                modified_buffer = true;
            } else {
                app.view.forget_buffer(buf)?;
            }
        }

        if modified_buffer {
            // Display a confirmation prompt before closing a modified buffer.
            let confirm_mode = ConfirmMode::new(close_others_confirm);
            app.mode = Mode::Confirm(confirm_mode);
            break;
        }

        // We haven't broken from the loop, so we're not back
        // at the original buffer; close the current buffer.
        app.workspace.close_current_buffer();
    }

    Ok(())
}

pub fn close_others_confirm(app: &mut Application) -> Result {
    if let Some(buf) = app.workspace.current_buffer() {
        app.view.forget_buffer(buf)?;
    }
    app.workspace.close_current_buffer();
    commands::application::switch_to_normal_mode(app)?;

    Ok(())
}

pub fn backspace(app: &mut Application) -> Result {
    let mut outdent = false;

    if let Some(buffer) = app.workspace.current_buffer() {
        if buffer.cursor.offset == 0 {
            buffer.cursor.move_up();
            buffer.cursor.move_to_end_of_line();
            buffer.delete();
        } else {
            let data = buffer.data();
            let current_line = data
                .lines()
                .nth(buffer.cursor.line)
                .ok_or(CURRENT_LINE_MISSING)?;
            if current_line.chars().all(|c| c.is_whitespace()) {
                outdent = true
            } else {
                buffer.cursor.move_left();
                buffer.delete();
            }
        }
    } else {
        bail!(BUFFER_MISSING);
    }

    if outdent {
        commands::buffer::outdent_line(app)?;
    }
    commands::view::scroll_to_cursor(app)
}

pub fn insert_char(app: &mut Application) -> Result {
    if let Some(buffer) = app.workspace.current_buffer() {
        if let Some(Key::Char(character)) = *app.view.last_key() {
            // TODO: Drop explicit call to to_string().
            buffer.insert(character.to_string());
            buffer.cursor.move_right();
        } else {
            bail!("No character to insert");
        }
    } else {
        bail!(BUFFER_MISSING);
    }
    commands::view::scroll_to_cursor(app)?;

    Ok(())
}

pub fn display_current_scope(app: &mut Application) -> Result {
    let scope_display_buffer = {
        let buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;
        let scope_stack = buffer.current_scope().chain_err(|| "No syntax definition for the current buffer")?;
        let mut scope_display_buffer = Buffer::new();
        for scope in scope_stack.as_slice().iter() {
            scope_display_buffer.insert(
                format!("{}\n", scope.build_string())
            );
        }

        scope_display_buffer
    };
    util::add_buffer(scope_display_buffer, app)
}

/// Inserts a newline character at the current cursor position.
/// Also performs automatic indentation, basing the indent off
/// of the previous line's leading whitespace.
pub fn insert_newline(app: &mut Application) -> Result {
    if let Some(buffer) = app.workspace.current_buffer() {
        // Insert the newline character.
        buffer.insert("\n");

        // Get the cursor position before moving it to the start of the new line.
        let position = buffer.cursor.clone();
        buffer.cursor.move_down();
        buffer.cursor.move_to_start_of_line();

        // Get a slice of the buffer up to and including the current line.
        let data = buffer.data();
        let end_of_current_line = data
            .lines()
            .nth(position.line)
            .map(|l| (l.as_ptr() as usize) + l.len())
            .unwrap();
        let offset = end_of_current_line - (data.as_str().as_ptr() as usize);
        let (previous_content, _) = data.split_at(offset);

        // Searching backwards, copy the nearest non-blank line's indent content.
        let nearest_non_blank_line = previous_content.lines().rev().find(|line| !line.is_empty());
        let indent_content = match nearest_non_blank_line {
            Some(line) => line.chars().take_while(|&c| c.is_whitespace()).collect(),
            None => String::new(),
        };

        // Insert and move to the end of the indent content.
        let indent_length = indent_content.chars().count();
        buffer.insert(indent_content);
        buffer.cursor.move_to(Position {
            line: position.line + 1,
            offset: indent_length,
        });
    } else {
        bail!(BUFFER_MISSING);
    }
    commands::view::scroll_to_cursor(app)?;

    Ok(())
}

pub fn indent_line(app: &mut Application) -> Result {
    let buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;
    let tab_content = app.preferences.borrow().tab_content(buffer.path.as_ref());

    let target_position = match app.mo