use crate::models::application::Event;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::view::Terminal;

pub struct EventListener {
    terminal: Arc<Box<dyn Terminal + Sync + Send + 'static>>,
    events: Sender<Event>,
    killswitch: Receiver<()>
}

impl EventListener {
    /// Spins up a thread that loops forever, waiting on terminal events
    /// and forwarding those to the application event channel.
    pub fn start(terminal: Arc<Box<dyn Terminal + Sync + Send + 'static>>, events: Sender<Event>, killswitch: Receiver<()>) {
