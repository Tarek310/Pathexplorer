use crate::controller::{AppEvents, State};
use crate::file_manager::FileManager;
use crate::util;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Alignment::Center;
use ratatui::prelude::{Style, Stylize};
use ratatui::widgets::{Block, List};

pub struct KeyMappingPopup;

impl KeyMappingPopup {
    pub fn new() -> KeyMappingPopup {
        KeyMappingPopup
    }
}

impl State for KeyMappingPopup {
    fn enter(&mut self, file_manager: &mut FileManager) {}

    fn exit(&mut self, file_manager: &mut FileManager) {}

    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        file_manager: &mut FileManager,
    ) -> AppEvents {
        AppEvents::ChangeToExplorerWindow
    }

    fn draw(&mut self, frame: &mut Frame, _file_manager: &mut FileManager) {
        let area = frame.area();

        //let vertical = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        //let [instructions, content] = vertical.areas(area);

        let popup_block = Block::bordered()
            .title("KEY MAPPINGS")
            .title_alignment(Center);
        let popup_area = util::popup_area(area, 20, 30);

        let list = List::new(vec![
            "<c> → copy".to_owned(),
            "<shift + c> → add to copy".to_owned(),
            "<v> → paste".to_owned(),
            "<x> → delete".to_owned(),
            "<h> → toggle hidden files".to_owned(),
            "<d> → change folder positions".to_owned(),
            "<s> → open sorting popup".to_owned(),
            "<q> → quit file explorer".to_owned(),
        ])
        .block(popup_block)
        .highlight_style(Style::new().red());

        frame.render_widget(list, popup_area);
    }
}
