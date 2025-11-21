use crate::explorer_table::ExplorerTable;
use crate::file_manager::FileManager;
use crate::key_mapping_popup::KeyMappingPopup;
use crate::sorting_popup::SortingPopUp;
use crossterm::event;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use ratatui::Frame;
use std::io;

pub enum AppEvents {
    None,
    Exit,
    ChangeToSortingPopupWindow,
    ChangeToExplorerWindow,
    ChangeToKeyMappingPopupWindow,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum AppWindows {
    Explorer = 0,
    SortingPopup = 1,
    KeyMappingPopup = 2,
}

pub trait State {
    fn enter(&mut self, file_manager: &mut FileManager);
    fn exit(&mut self, file_manager: &mut FileManager);
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        file_manager: &mut FileManager,
    ) -> AppEvents;
    fn draw(&mut self, frame: &mut Frame, file_manager: &mut FileManager);
}

pub struct Controller {
    pub all_states: [Box<dyn State>; 3],
    pub current_state_index: AppWindows,
    pub file_manager: FileManager,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            all_states: [
                Box::new(ExplorerTable::new()),
                Box::new(SortingPopUp::new()),
                Box::new(KeyMappingPopup::new()),
            ],
            current_state_index: AppWindows::Explorer,
            file_manager: FileManager::new(),
        }
    }
    pub fn change_state(&mut self, new_window: AppWindows) {
        self.all_states[self.current_state_index as usize].exit(&mut self.file_manager);
        self.current_state_index = new_window;
        self.all_states[self.current_state_index as usize].enter(&mut self.file_manager);
    }

    pub fn handle_events(&mut self) -> io::Result<AppEvents> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                let app_event = self.all_states[self.current_state_index as usize]
                    .handle_key_event(key_event, &mut self.file_manager);
                match app_event {
                    AppEvents::None => Ok(AppEvents::None),
                    AppEvents::Exit => Ok(AppEvents::Exit),
                    AppEvents::ChangeToSortingPopupWindow => {
                        self.change_state(AppWindows::SortingPopup);
                        Ok(AppEvents::None)
                    }
                    AppEvents::ChangeToExplorerWindow => {
                        self.change_state(AppWindows::Explorer);
                        Ok(AppEvents::None)
                    }
                    AppEvents::ChangeToKeyMappingPopupWindow => {
                        self.change_state(AppWindows::KeyMappingPopup);
                        Ok(AppEvents::None)
                    }
                }
            }
            _ => Ok(AppEvents::None),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.all_states[self.current_state_index as usize].draw(frame, &mut self.file_manager);
    }
}
