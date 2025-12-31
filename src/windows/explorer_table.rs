use crate::controller::{AppEvents, State};
use crate::file_manager::{FileManager, SortDir};
use crate::message::{Message, MessageReceiver, MessageSender};
use crate::string_ring_buffer::StringRingBuffer;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Line, Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph, Row, Table, TableState, Wrap};
use std::path::PathBuf;

//this enum is used to know which part of the window requested the popup to properly handle the
//message
enum MessageSource {
    None,
    DeletionConfirmationPrompt,
    PathChangePopup,
}

pub struct ExplorerTable {
    table_state: TableState,
    message_source: MessageSource,
    message: Option<Message>,

    error_ring_buffer: StringRingBuffer,
}

impl ExplorerTable {
    pub fn new() -> ExplorerTable {
        let mut explorer_table = ExplorerTable {
            table_state: TableState::new(),
            message_source: MessageSource::None,
            message: None,

            error_ring_buffer: StringRingBuffer::with_capacity(20),
        };
        explorer_table.table_state.select_first_column();
        explorer_table.table_state.select_first();
        explorer_table
    }

    /// get the corresponding file to the one that is selected in table_state.
    /// Returns Option with the PathBuf or None if no File is currently selected
    pub fn selected_file_in_table(&self, file_manager: &mut FileManager) -> Option<PathBuf> {
        let index = self.table_state.selected()?;
        let entry = match file_manager.get_entry_at_index(index) {
            Ok(entry) => entry,
            Err(_e) => {
                return None;
            }
        };
        Some(entry.path())
    }
}

impl MessageReceiver for ExplorerTable {
    fn handle_message(
        &mut self,
        message: Option<Message>,
        file_manager: &mut crate::file_manager::FileManager,
    ) {
        match self.message_source {
            MessageSource::DeletionConfirmationPrompt => {
                if let Some(Message::Bool(true)) = message {
                    file_manager.delete_selection();
                }
            }
            MessageSource::PathChangePopup => {
                if let Some(Message::String(path_string)) = message {
                    let new_path = PathBuf::from(path_string);

                    file_manager.change_dir_with_error_handling(new_path);

                    if self.table_state.selected().is_none() {
                        self.table_state.select(Some(0));
                    }
                }
                self.message_source = MessageSource::None;
            }
            MessageSource::None => {}
        }
    }
}
impl MessageSender for ExplorerTable {
    fn get_message(&mut self) -> Option<Message> {
        self.message.take()
    }
}

impl State for ExplorerTable {
    fn enter(&mut self, file_manager: &mut FileManager) {
        file_manager.update();
    }

    fn exit(&mut self, _file_manager: &mut FileManager) {}

    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        file_manager: &mut FileManager,
    ) -> AppEvents {
        match key_event.code {
            KeyCode::Char('q') => return AppEvents::Exit,
            KeyCode::Char('s') => {
                return AppEvents::OpenSortingPopupWindow;
            }
            KeyCode::Char('m') => {
                return AppEvents::OpenKeyMappingPopupWindow;
            }
            KeyCode::Char('n') => {
                return AppEvents::OpenNewFilePopup;
            }
            KeyCode::Char('d') => {
                match file_manager.dir_sorting {
                    SortDir::Unsorted => file_manager.dir_sorting = SortDir::Start,
                    SortDir::Start => file_manager.dir_sorting = SortDir::End,
                    SortDir::End => file_manager.dir_sorting = SortDir::Unsorted,
                }
                file_manager.update();
                return AppEvents::None;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let selected = self.table_state.selected();
                match selected {
                    None => self.table_state.select_last(),
                    Some(selected) => {
                        if file_manager.num_files - 1 <= selected {
                            self.table_state.select_first();
                        } else {
                            self.table_state.select_next();
                        }
                    }
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let selected = self.table_state.selected();
                match selected {
                    None => self.table_state.select_last(),
                    Some(selected) => {
                        if selected == 0 {
                            self.table_state.select_last();
                        } else {
                            self.table_state.select_previous();
                        }
                    }
                }
            }
            //Enter selected Directory
            KeyCode::Right | KeyCode::Char('l') => {
                let index = match self.table_state.selected() {
                    Some(index) => index,
                    None => return AppEvents::None,
                };
                let entry = file_manager.get_entries().get(index).unwrap();
                file_manager.change_dir_with_error_handling(entry.path());
            }
            //Go to parent directory
            KeyCode::Left | KeyCode::Char('h') => {
                file_manager.change_dir_with_error_handling(PathBuf::from(".."));
                if self.table_state.selected().is_none() {
                    self.table_state.select(Some(0));
                }
            }

            //toggle file/folder selection
            KeyCode::Char('y') => {
                let path = match self.selected_file_in_table(file_manager) {
                    None => return AppEvents::None,
                    Some(path) => path,
                };
                if file_manager.is_selected(&path) {
                    file_manager.remove_from_selection(path);
                } else {
                    file_manager.add_to_selection(path);
                }
            }

            //clear selection
            KeyCode::Char('c') => {
                file_manager.clear_selection();
            }
            //paste selection
            KeyCode::Char('v') => {
                file_manager.paste();
                file_manager.clear_selection();
            }

            //delete selection
            KeyCode::Char('x') => {
                self.message_source = MessageSource::DeletionConfirmationPrompt;
                self.message = Some(Message::String(
                    "The selected files will be deleted permanently, are you sure?".to_owned(),
                ));
                return AppEvents::OpenConfirmationPopup;
            }
            KeyCode::Char('g') => {
                file_manager.show_hidden = !file_manager.show_hidden;
                file_manager.update();
            }
            KeyCode::Tab => {
                // Get current directory path
                let current_path = match file_manager.current_dir() {
                    Ok(path) => path.into_os_string().into_string().unwrap_or_default(),
                    Err(_) => String::from(""),
                };

                // Set message source and message
                self.message_source = MessageSource::PathChangePopup;
                self.message = Some(Message::TwoStrings(
                    String::from("Change Path"),
                    current_path,
                ));

                return AppEvents::OpenTextFieldPopup;
            }
            KeyCode::Enter => {
                let path = match self.selected_file_in_table(file_manager) {
                    None => return AppEvents::None,
                    Some(path) => path,
                };
                file_manager.open_path(&path);
            }

            _ => {}
        }
        AppEvents::None
    }

    fn draw(&mut self, frame: &mut Frame, file_manager: &mut FileManager) {
        // Update error log
        for x in file_manager.take_errors() {
            self.error_ring_buffer.push(x.to_string());
        }

        let title = Line::from("FILE EXPLORER").bold();
        let help_text = Line::from("Key Mappings:<m>");
        let table_block = Block::bordered()
            .title(title.left_aligned().bold())
            .border_set(border::THICK)
            .title_bottom(help_text.right_aligned().bold());

        let path_block = Block::bordered().title("PATH").border_set(border::THICK);
        let error_log_block = Block::bordered()
            .title("ERROR LOG")
            .border_set(border::THICK);

        let horizontal_layout =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

        let path_area = horizontal_layout[0];
        let main_area = horizontal_layout[1];

        let vertical_layout =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(main_area);

        let table_area = vertical_layout[0];
        let error_area = vertical_layout[1];

        let inner_path_area = path_block.inner(path_area);
        let inner_error_area = error_log_block.inner(error_area);

        let error_log_paragraph = Paragraph::new(self.error_ring_buffer.to_string()).wrap(Wrap {
            ..Default::default()
        });

        //write path to path_area
        let path = file_manager
            .current_dir()
            .unwrap_or_default()
            .into_os_string()
            .into_string()
            .unwrap_or_default();

        let text_paragraph = Paragraph::new(path).left_aligned().wrap(Wrap {
            ..Default::default()
        });

        let mut rows: Vec<Row> = Vec::new();
        let header = Row::new(vec!["FILENAME", "SIZE"]).bold().dark_gray();
        for entry in file_manager.get_entries() {
            let mut row_strings: Vec<String> = Vec::new();
            row_strings.push(entry.file_name().into_string().unwrap());
            if entry.metadata().unwrap().is_file() {
                row_strings.push(entry.metadata().unwrap().len().to_string());
            } else {
                row_strings.push("".to_string());
            }
            let mut row = Row::new(row_strings);
            if file_manager.is_selected(&entry.path()) {
                row = row.on_dark_gray();
            } else if entry.metadata().unwrap().is_dir() {
                row = row.blue();
            }
            rows.push(row);
        }
        let widths = [Constraint::Percentage(20), Constraint::Percentage(20)];

        let table = Table::new(rows, widths)
            .block(table_block)
            .header(header)
            .cell_highlight_style(Style::new().green());

        frame.render_stateful_widget(table, table_area, &mut self.table_state);
        frame.render_widget(error_log_paragraph, inner_error_area);
        frame.render_widget(text_paragraph, inner_path_area);
        frame.render_widget(error_log_block, error_area);
        frame.render_widget(path_block, path_area);
    }
}
