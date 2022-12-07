use crate::models::application::Event;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::view::