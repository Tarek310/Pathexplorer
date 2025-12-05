use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Clear, List, ListState, Paragraph, Wrap},
};

use crate::{
    controller::{AppEvents, State},
    file_manager::FileManager,
    message::{Message, MessageReceiver, MessageSender},
    util,
};

pub struct ConfirmationPopup {
    text: String,
    confirmation_result: Option<bool>,
    list_state: ListState,
}

impl ConfirmationPopup {
    pub fn new(message: Option<Message>, file_manager: &mut FileManager) -> Self {
        let mut confirmation_popup = ConfirmationPopup {
            text: "".to_owned(),
            confirmation_result: None,
            list_state: ListState::default(),
        };
        confirmation_popup.list_state.select(Some(0));
        confirmation_popup.handle_message(message, file_manager);
        confirmation_popup
    }
}

impl State for ConfirmationPopup {
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        _file_manager: &mut FileManager,
    ) -> AppEvents {
        match key_event.code {
            KeyCode::Enter => {
                match self.list_state.selected() {
                    Some(0) => self.confirmation_result = Some(false),
                    _ => self.confirmation_result = Some(true),
                }
                return AppEvents::ClosePopUp;
            }
            KeyCode::Esc => return AppEvents::ClosePopUp,
            KeyCode::Up | KeyCode::Char('k') => self.list_state.select(Some(0)),
            KeyCode::Down | KeyCode::Char('j') => self.list_state.select(Some(1)),
            _ => {}
        }
        AppEvents::None
    }

    fn draw(
        &mut self,
        frame: &mut ratatui::Frame,
        _file_manager: &mut crate::file_manager::FileManager,
    ) {
        let area = frame.area();

        let popup_block_text = Block::bordered();
        let popup_block_selection = Block::bordered();
        let popup_area = util::popup_area(area, 40, 25);

        let layout =
            Layout::vertical([Constraint::Min(3), Constraint::Length(4)]).split(popup_area);

        let text_area = layout[0];
        let list_area = layout[1];

        let text_inner = popup_block_text.inner(text_area);
        let list_inner = popup_block_selection.inner(list_area);

        let text_paragraph = Paragraph::new(self.text.as_str()).centered().wrap(Wrap {
            ..Default::default()
        });

        let list = List::new(vec![
            Line::from("No").centered(),
            Line::from("Yes").centered(),
        ])
        .highlight_style(Style::new().blue());

        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_block_text, text_area);
        frame.render_widget(popup_block_selection, list_area);
        frame.render_widget(text_paragraph, text_inner);
        frame.render_stateful_widget(list, list_inner, &mut self.list_state);
    }
}

impl MessageReceiver for ConfirmationPopup {
    fn handle_message(
        &mut self,
        message: Option<Message>,
        _file_manager: &mut crate::file_manager::FileManager,
    ) {
        if let Some(Message::String(message)) = message {
            self.text = message;
        }
    }
}
impl MessageSender for ConfirmationPopup {
    fn get_message(&mut self) -> Option<Message> {
        if let Some(index) = self.list_state.selected() {
            match index {
                0 => Some(Message::Bool(false)),
                _ => Some(Message::Bool(true)),
            }
        } else {
            None
        }
    }
}
