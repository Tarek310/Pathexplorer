use crate::app::App;
use crate::file_manager::FileManager;
use crate::message::{self, Message, MessageReceiver, MessageSender};
use crate::windows::explorer_table::ExplorerTable;
use crate::windows::popups::confirmation_popup::ConfirmationPopup;
use crate::windows::popups::key_mapping_popup::KeyMappingPopup;
use crate::windows::popups::new_file_popup::NewFilePopup;
use crate::windows::popups::sorting_popup::SortingPopUp;
use crate::windows::popups::text_field_popup::TextFieldPopup;
use crossterm::event;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use ratatui::Frame;
use std::io;

pub enum AppEvents {
    None,
    Exit,
    OpenSortingPopupWindow,
    ChangeToExplorerWindow,
    OpenKeyMappingPopupWindow,
    OpenTextFieldPopup,
    OpenNewFilePopup,
    OpenConfirmationPopup,
    ClosePopUp,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum AppWindows {
    Explorer = 0,
}

pub trait State: MessageReceiver + MessageSender {
    fn enter(&mut self, _file_manager: &mut FileManager) {}
    fn exit(&mut self, _file_manager: &mut FileManager) {}
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        file_manager: &mut FileManager,
    ) -> AppEvents;
    fn draw(&mut self, frame: &mut Frame, file_manager: &mut FileManager);
}

pub struct Controller {
    pub all_windows: [Box<dyn State>; 1],
    pub current_window_index: AppWindows,
    pub popup_stack: Vec<Box<dyn State>>,
    pub file_manager: FileManager,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            all_windows: [Box::new(ExplorerTable::new())],
            current_window_index: AppWindows::Explorer,
            popup_stack: Vec::new(),
            file_manager: FileManager::new(),
        }
    }

    pub fn change_window(&mut self, new_window: AppWindows) {
        self.all_windows[self.current_window_index as usize].exit(&mut self.file_manager);
        self.current_window_index = new_window;
        self.all_windows[self.current_window_index as usize].enter(&mut self.file_manager);
    }

    pub fn handle_events(&mut self) -> io::Result<AppEvents> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                //if popups are active, handle popup instead of window
                let app_event: AppEvents = if !self.popup_stack.is_empty() {
                    self.popup_stack
                        .last_mut()
                        .unwrap()
                        .handle_key_event(key_event, &mut self.file_manager)
                } else {
                    self.all_windows[self.current_window_index as usize]
                        .handle_key_event(key_event, &mut self.file_manager)
                };

                match app_event {
                    AppEvents::None => Ok(AppEvents::None),
                    AppEvents::Exit => Ok(AppEvents::Exit),
                    AppEvents::OpenSortingPopupWindow => {
                        self.popup_stack.push(Box::new(SortingPopUp::new()));
                        Ok(AppEvents::None)
                    }
                    AppEvents::ChangeToExplorerWindow => {
                        self.change_window(AppWindows::Explorer);
                        Ok(AppEvents::None)
                    }
                    AppEvents::OpenKeyMappingPopupWindow => {
                        self.popup_stack.push(Box::new(KeyMappingPopup::new()));
                        Ok(AppEvents::None)
                    }
                    AppEvents::OpenTextFieldPopup => {
                        let message = self.get_current_message();
                        self.popup_stack.push(Box::new(TextFieldPopup::new()));
                        self.send_current_message(message);
                        Ok(AppEvents::None)
                    }
                    AppEvents::OpenConfirmationPopup => {
                        let message = self.get_current_message();
                        self.popup_stack.push(Box::new(ConfirmationPopup::new()));
                        self.send_current_message(message);
                        Ok(AppEvents::None)
                    }

                    AppEvents::OpenNewFilePopup => {
                        self.popup_stack.push(Box::new(NewFilePopup::new()));
                        Ok(AppEvents::None)
                    }

                    AppEvents::ClosePopUp => {
                        assert!(!self.popup_stack.is_empty());
                        //pass down message
                        let message = self.get_current_message();
                        self.popup_stack.pop();
                        self.send_current_message(message);
                        Ok(AppEvents::None)
                    }
                }
            }
            _ => Ok(AppEvents::None),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        //Draw base window then all popups

        self.all_windows[self.current_window_index as usize].draw(frame, &mut self.file_manager);
        for x in &mut self.popup_stack {
            x.draw(frame, &mut self.file_manager);
        }
    }

    /// Get message from currently active window
    pub fn get_current_message(&mut self) -> Option<Message> {
        if !self.popup_stack.is_empty() {
            self.popup_stack.last_mut().unwrap().get_message()
        } else {
            self.all_windows[self.current_window_index as usize].get_message()
        }
    }

    /// Send message to currently active window
    pub fn send_current_message(&mut self, message: Option<Message>) {
        if !self.popup_stack.is_empty() {
            self.popup_stack
                .last_mut()
                .unwrap()
                .handle_message(message, &mut self.file_manager);
        } else {
            self.all_windows[self.current_window_index as usize]
                .handle_message(message, &mut self.file_manager);
        }
    }
}
