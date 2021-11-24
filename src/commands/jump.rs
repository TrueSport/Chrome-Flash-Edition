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
                }