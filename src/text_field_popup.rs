use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::{
    controller::{AppEvents, State},
    file_manager::FileManager,
    message::{self, Message, MessageReceiver, MessageSender},
    util,
};

///This popup is for retrieving a String from the user.
///The controller will pass the message to the window that requested this popup
pub struct TextFieldPopup {
    string: String,
    message: String,
}

impl TextFieldPopup {
    pub fn new() -> TextFieldPopup {
        TextFieldPopup {
            string: String::from(""),
            message: String::from(""),
        }
    }
}

impl MessageReceiver for TextFieldPopup {}

impl MessageSender for TextFieldPopup {
    fn get_message(&mut self) -> Option<Message> {
        if self.message.is_empty() {
            None
        } else {
            Some(Message::String(String::from(&self.message)))
        }
    }
}

impl State for TextFieldPopup {
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        _file_manager: &mut FileManager,
    ) -> AppEvents {
        match key_event.code {
            KeyCode::Char(c) => self.string.push(c),
            KeyCode::Backspace => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.string.clear();
                } else {
                    self.string.pop();
                }
            }
            KeyCode::Esc => {
                self.string.clear();
                return AppEvents::ClosePopUp;
            }
            KeyCode::Enter => {
                self.message = String::from(&self.string);
                self.string.clear();
                return AppEvents::ClosePopUp;
            }
            _ => {}
        };
        AppEvents::None
    }

    fn draw(
        &mut self,
        frame: &mut ratatui::Frame,
        _file_manager: &mut crate::file_manager::FileManager,
    ) {
        let area = frame.area();
        let popup_block = Block::bordered().title("file/folder name:");
        let popup_area = util::popup_area(area, 40, 10);
        let paragraph = Paragraph::new(String::from(&self.string)).block(popup_block);

        frame.render_widget(Clear, popup_area);
        frame.render_widget(paragraph, popup_area);

        frame.set_cursor_position((
            popup_area.x + 1 + self.string.chars().count() as u16,
            popup_area.y + 1,
        ));
    }
}

