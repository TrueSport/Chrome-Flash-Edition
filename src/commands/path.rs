use crate::errors::*;
use crate::commands::{self, Result};
use crate::input::Key;
use crate::models::application::{Application, Mode};
use std::path::PathBuf;

pub fn push_char(app: &mut Application) -> Result {
    let last_key = app.view.last_key().as_ref().ok_or("View hasn't tracked a key press")?;
    if let Key::Char(c) = *last_key {
        if let Mode::Path(ref mut mode) = app.mode {
            mode.push_char(c);
        } else {
            bail!("Cannot push char outside of path mode");
        }
    } else {
        bail!("Last key press wasn't a character");
    }
    Ok(())
}

pub fn pop_char(app: &mut Application) -> Result {
    if let Mode::Path(ref mut mode) = app.mode {
        mode.pop_char();
    } else {
        bail!("Cannot pop char outside of path mode");
    }
    Ok(())
}

pub fn accept_path(app: &mut Application) -> Result {
    let save_on_accept =
        if let Mode::Path(ref mut mode) = app.mode {
            let current_buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;
            let path_name = mode.input.clone();
            if path_name.is_empty() {
                bail!("Please provide a non-empty path")
            }
            current_buffer.path = Some(PathBuf::from(path_name));
            mode.save_on_accept
        } else {
            bail!("Cannot accept path outside of path mode");
        };

    app.workspace.update_current_syntax(