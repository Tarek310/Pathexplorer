use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    style::{Style, Stylize},
    widgets::{Block, Clear, List, ListState},
};

use crate::{
    controller::{AppEvents, State},
    file_manager::FileManager,
    message::{Message, MessageReceiver, MessageSender},
    util,
};

pub struct NewFilePopup {
    list_state: ListState,
}

impl NewFilePopup {
    pub fn new(message: Option<Message>, file_manager: &mut FileManager) -> NewFilePopup {
        let mut popup = NewFilePopup {
            list_state: ListState::default(),
        };
        popup.list_state.select(Some(0));
        popup.handle_message(message, file_manager);
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
            let path = PathBuf::from(message);
            match index {
                0 => {
                    let _ = file_manager.create_file(path);
                }
                1 => {
                    let _ = file_manager.create_folder(path);
                }
                _ => {}
            }
        }
        file_manager.update();
    }
}

impl MessageSender for NewFilePopup {
    fn get_message(&mut self) -> Option<Message> {
        let index = self.list_state.selected().unwrap();
        match index {
            0 => Some(Message::String(String::from("File"))),
            1 => Some(Message::String(String::from("Folder"))),
            _ => None,
        }
    }
}
impl State for NewFilePopup {
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        _file_manager: &mut FileManager,
    ) -> AppEvents {
        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => self.list_state.select_previous(),
            KeyCode::Down | KeyCode::Char('j') => self.list_state.select_next(),
            KeyCode::Esc => return AppEvents::ClosePopUp,
            KeyCode::Enter => return AppEvents::OpenTextFieldPopup,
            _ => {}
        }
        AppEvents::None
    }

    fn draw(&mut self, frame: &mut Frame, _file_manager: &mut FileManager) {
        let area = frame.area();

        let popup_block = Block::bordered().title("Create:");
        let popup_area = util::popup_area(area, 20, 20);

        let list = List::new(vec!["File".to_owned(), "Folder".to_owned()])
            .block(popup_block)
            .highlight_style(Style::new().red());

        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(list, popup_area, &mut self.list_state);
    }
}
