use crate::controller::{AppEvents, State};
use crate::file_manager::FileManager;
use crate::message::{MessageReceiver, MessageSender};
use crate::util;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Alignment::Center;
use ratatui::prelude::{Style, Stylize};
use ratatui::widgets::{Block, Clear, List};

pub struct KeyMappingPopup;

impl KeyMappingPopup {
    pub fn new() -> KeyMappingPopup {
        KeyMappingPopup
    }
}

impl MessageReceiver for KeyMappingPopup {}
impl MessageSender for KeyMappingPopup {}

impl State for KeyMappingPopup {
    fn handle_key_event(
        &mut self,
        _key_event: KeyEvent,
        _file_manager: &mut FileManager,
    ) -> AppEvents {
        AppEvents::ClosePopUp
    }

    fn draw(&mut self, frame: &mut Frame, _file_manager: &mut FileManager) {
        let area = frame.area();

        //let vertical = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        //let [instructions, content] = vertical.areas(area);

        let popup_block = Block::bordered()
            .title("KEY MAPPINGS")
            .title_alignment(Center);
        let mut popup_area = util::popup_area(area, 20, 30);

        let list = List::new(vec![
            "<c> → clear selection".to_owned(),
            "<v> → paste selection".to_owned(),
            "<x> → delete selection".to_owned(),
            "<y> → add to selection".to_owned(),
            "<g> → toggle hidden files".to_owned(),
            "<d> → change folder positions".to_owned(),
            "<s> → open sorting popup".to_owned(),
            "<q> → quit file explorer".to_owned(),
            "<n> → create new file".to_owned(),
        ])
        .block(popup_block)
        .highlight_style(Style::new().red());

        popup_area.height = list.len() as u16 + 2;
        frame.render_widget(Clear, popup_area);
        frame.render_widget(list, popup_area);
    }
}
