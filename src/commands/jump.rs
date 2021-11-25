use crate::errors::*;
use crate::input::Key;
use std::mem;
use crate::commands::Result;
use crate::models::application::modes::jump;
use crate::models::application::modes::JumpMode;
use crate::models::application::{Mode, Application};
use scribe::Workspace;

pub fn match_tag(app: &mut Application) -> Result {
    let result =
        if let Mode::Jump(ref mut jump_mode) = app.mode {
            match jump_mode.input.len() {
                0 => return Ok(()), // Not enough data to match to a position.
                1 => {
                    if jump_mode.first_phase {
                        jump_to_tag(jump_mode, &mut app.workspace)
                    } else {
                        return Ok(()) // Not enough data to match to a position.
                    }
                },
                _ => jump_to_tag(jump_mode, &mut app.workspace),
            }
        } else {
            bail!("Can't match jump tags outside of jump mode.");
        };
    switch_to_previous_mode(app);

    result
}

// Try to find a position for the input tag and jump to it.
fn jump_to_tag(jump_mode: &mut JumpMode, workspace: &mut Workspace) -> Result {
    let position = jump_mode
        .map_tag(&jump_mode.input)
        .ok_or("Couldn't find a position for the specified tag")?;
    let buffer = workspace.current_buffer().ok_or(BUFFER_MISSING)?;

    if !buffer.cursor.move_to(*position) {
        bail!("Couldn't move to the specified tag's position ({:?})", position)
    }

    Ok(())
}

fn switch_to_previous_mode(app: &mut Application) {
    let old_mode = mem::replace