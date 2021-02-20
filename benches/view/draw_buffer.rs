extern crate amp;
#[macro_use]
extern crate criterion;

use amp::Application;
use criterion::Criterion;
use std::path::PathBuf;

fn buffer_rendering(c: &mut Criterion) {
    let mut app = Application::new(&Vec::new()).unwrap();
    app.workspace.open_buffer(
        &PathBuf::from("src/commands/buffer.rs")
    ).unwrap();
    app.view.initialize_buffer(app.workspace.current_buffer().unwrap()).unwra