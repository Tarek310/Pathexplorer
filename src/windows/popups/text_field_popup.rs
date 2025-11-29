use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::{Block, Clear, Paragraph};

use crate::{
    controller::{AppEvents, State},
    file_manager::FileManager,
    message::{Message, MessageReceiver, MessageSender},
};

///This popup is for retrieving a String from the user.
///The controller will pass the message to the window that requested this popup
pub struct TextFieldPopup {
    title: String,
    string: String,
    message: String,
}

impl TextFieldPopup {
    pub fn new() -> TextFieldPopup {
        TextFieldPopup {
            title: String::from(""),
            string: String::from(""),
            message: String::from(""),
        }
    }
}

impl MessageReceiver for TextFieldPopup {
    fn handle_message(&mut self, message: Option<Message>, _file_manager: &mut FileManager) {
        if let Some(Message::String(message)) = message {
            self.title = message;
        }
    }
}

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
        let popup_block = Block::bordered().title(format!("{} name:", self.title));

        let vertical = ratatui::layout::Layout::vertical([ratatui::layout::Constraint::Length(3)])
            .flex(ratatui::layout::Flex::Center);
        let horizontal =
            ratatui::layout::Layout::horizontal([ratatui::layout::Constraint::Percentage(50)])
                .flex(ratatui::layout::Flex::Center);
        let [popup_area] = vertical.areas(area);
        let [popup_area] = horizontal.areas(popup_area);

        let paragraph = Paragraph::new(String::from(&self.string)).block(popup_block);

        frame.render_widget(Clear, popup_area);
        frame.render_widget(paragraph, popup_area);

        frame.set_cursor_position((
            popup_area.x + 1 + self.string.chars().count() as u16,
            popup_area.y + 1,
        ));
    }
}
