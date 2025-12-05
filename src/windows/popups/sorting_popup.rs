use crate::controller::{AppEvents, State};
use crate::file_manager::{FileManager, Sorting};
use crate::message::{MessageReceiver, MessageSender};
use crate::util;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Clear, List, ListState};

pub struct SortingPopUp {
    list_state: ListState,
}

impl SortingPopUp {
    pub fn new(
        message: Option<crate::message::Message>,
        file_manager: &mut FileManager,
    ) -> SortingPopUp {
        let mut popup = SortingPopUp {
            list_state: ListState::default(),
        };
        popup.list_state.select(Some(0));
        popup.handle_message(message, file_manager);
        popup
    }

    fn selected_sort_mode(&self) -> Option<Sorting> {
        let index = self.list_state.selected()?;

        match index {
            0 => Some(Sorting::SortedBySizeDescending),
            1 => Some(Sorting::SortedBySizeAscending),
            2 => Some(Sorting::SortedByNameDescending),
            3 => Some(Sorting::SortedByNameAscending),
            _ => None,
        }
    }
}

impl MessageReceiver for SortingPopUp {}
impl MessageSender for SortingPopUp {}

impl State for SortingPopUp {
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        file_manager: &mut FileManager,
    ) -> AppEvents {
        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => self.list_state.select_previous(),
            KeyCode::Down | KeyCode::Char('j') => self.list_state.select_next(),
            KeyCode::Enter => match self.selected_sort_mode() {
                None => return AppEvents::None,
                Some(sorting) => {
                    file_manager.sort(sorting);
                    return AppEvents::ClosePopUp;
                }
            },
            KeyCode::Esc => return AppEvents::ClosePopUp,
            _ => {}
        };
        AppEvents::None
    }

    fn draw(&mut self, frame: &mut Frame, _file_manager: &mut FileManager) {
        let area = frame.area();

        let popup_block = Block::bordered().title("sort by:");
        let popup_area = util::popup_area(area, 10, 30);

        let list = List::new(vec![
            "Size↓".to_owned(),
            "Size↑".to_owned(),
            "Name↓".to_owned(),
            "Name↑".to_owned(),
        ])
        .block(popup_block)
        .highlight_style(Style::new().red());

        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(list, popup_area, &mut self.list_state);
    }
}
