use crate::errors::*;
use crate::commands::{self, Result};
use std::mem;
use crate::input::Key;
use crate::util;
use crate::util::token::{Direction, adjacent_token_position};
use crate::models::application::{Application, ClipboardContent, Mode};
use crate::mo