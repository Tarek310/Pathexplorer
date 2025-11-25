mod app;
mod controller;
mod explorer_table;
mod file_manager;
mod key_mapping_popup;
mod message;
mod new_file_popup;
mod sorting_popup;
mod test;
mod text_field_popup;
mod util;

use crate::app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let res: io::Result<()> = App::new().run(&mut terminal);
    ratatui::restore();
    res
}
