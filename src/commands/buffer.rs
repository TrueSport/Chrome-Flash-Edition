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
        comma