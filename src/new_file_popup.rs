use std::fs::{self, File};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    style::{Style, Stylize},
    widgets::{Block, List, ListState},
};

use crate::{
    controller::{AppEvents, State},
    file_manager::{self, FileManager},
    message::{self, Message, MessageReceiver, MessageSender},
    util,
};

pub struct NewFilePopup {
    list_state: ListState,
}

impl NewFilePopup {
    pub fn new() -> NewFilePopup {
        let mut popup = NewFilePopup {
            list_state: ListState::default(),
        };
        popup.list_state.select(Some(0));
        popup
    }
}

impl MessageReceiver for NewFilePopup {
    fn handle_message(
        &mut self,
        message: Option<crate::message::Message>,
        file_manager: &mut crate::file_manager::FileManager,
    ) {
        if let Some(Message::String(message)) = message {
            let index = self.list_state.selected().unwrap();
            //TODO: move file/folder creation into file_manager
            match index {
                0 => match File::create(&message) {
                    Ok(_res) => {}
                    Err(_e) => todo!(),
                },
                1 => match fs::create_dir(&message) {
                    Ok(_res) => {}
                    Err(_e) => todo!(),
                },
                _ => {}
            }
        }
        file_manager.update();
    }
}

impl MessageSender for NewFilePopup {}
impl State for NewFilePopup {
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        _file_manager: &mut FileManager,
    ) -> AppEvents {
        match key_event.code {
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Esc => return AppEvents::ClosePopUp,
            KeyCode::Enter => return AppEvents::OpenTextFieldPopup,
            _ => {}
        }
        AppEvents::None
    }

    fn draw(&mut self, frame: &mut Frame, _file_manager: &mut FileManager) {
        let area = frame.area();

        let popup_block = Block::bordered().title("Create:");
        let popup_area = util::popup_area(area, 10, 30);

        let list = List::new(vec!["File".to_owned(), "Folder".to_owned()])
            .block(popup_block)
            .highlight_style(Style::new().red());

        frame.render_stateful_widget(list, popup_area, &mut self.list_state);
    }
}
