
use crate::errors::*;
use crate::commands::{self, Result};
use crate::util::token::{Direction, adjacent_token_position};
use crate::models::application::Application;
use scribe::buffer::Position;
use super::{application, buffer};

pub fn move_up(app: &mut Application) -> Result {
    app.workspace.current_buffer().ok_or(BUFFER_MISSING)?.cursor.move_up();
    commands::view::scroll_to_cursor(app).chain_err(|| SCROLL_TO_CURSOR_FAILED)
}

pub fn move_down(app: &mut Application) -> Result {
    app.workspace.current_buffer().ok_or(BUFFER_MISSING)?.cursor.move_down();
    commands::view::scroll_to_cursor(app).chain_err(|| SCROLL_TO_CURSOR_FAILED)
}

pub fn move_left(app: &mut Application) -> Result {
    app.workspace.current_buffer().ok_or(BUFFER_MISSING)?.cursor.move_left();
    commands::view::scroll_to_cursor(app).chain_err(|| SCROLL_TO_CURSOR_FAILED)
}

pub fn move_right(app: &mut Application) -> Result {
    app.workspace.current_buffer().ok_or(BUFFER_MISSING)?.cursor.move_right();
    commands::view::scroll_to_cursor(app).chain_err(|| SCROLL_TO_CURSOR_FAILED)
}

pub fn move_to_start_of_line(app: &mut Application) -> Result {
    app.workspace
        .current_buffer()
        .ok_or(BUFFER_MISSING)?
        .cursor
        .move_to_start_of_line();
    commands::view::scroll_to_cursor(app).chain_err(|| SCROLL_TO_CURSOR_FAILED)
}

pub fn move_to_end_of_line(app: &mut Application) -> Result {