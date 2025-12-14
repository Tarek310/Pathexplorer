mod app;
mod controller;
mod file_manager;
mod message;
mod string_ring_buffer;
mod test;
mod util;
mod windows;

use crate::app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let res: io::Result<()> = App::new().run(&mut terminal);
    ratatui::restore();
    res
}
