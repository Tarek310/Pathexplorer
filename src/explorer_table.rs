use crate::controller::{AppEvents, State};
use crate::file_manager::{FileManager, SortDir};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::prelude::{Line, Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Row, Table, TableState};
use std::path::PathBuf;

pub struct ExplorerTable {
    table_state: TableState,
}

impl ExplorerTable {
    pub fn new() -> ExplorerTable {
        let mut explorer_table = ExplorerTable {
            table_state: TableState::new(),
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
                return AppEvents::ChangeToSortingPopupWindow;
            }
            KeyCode::Char('m') => {
                return AppEvents::ChangeToKeyMappingPopupWindow;
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
            KeyCode::Down => {
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
            KeyCode::Up => {
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
            KeyCode::Right => {
                let index = match self.table_state.selected() {
                    Some(index) => index,
                    None => return AppEvents::None,
                };
                let entry = file_manager.get_entries().get(index).unwrap();
                if entry.metadata().unwrap().is_dir() {
                    file_manager.change_dir(entry.path());
                    if self.table_state.selected().is_none() {
                        self.table_state.select(Some(0));
                    }
                }
            }
            //Go to parent directory
            KeyCode::Left => {
                file_manager.change_dir(PathBuf::from(".."));
                if self.table_state.selected().is_none() {
                    self.table_state.select(Some(0));
                }
            }
            //copy
            KeyCode::Char('c') | KeyCode::Char('C') => {
                let path = match self.selected_file_in_table(file_manager) {
                    None => return AppEvents::None,
                    Some(path) => path,
                };
                if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                    file_manager.add_copy(path);
                } else {
                    file_manager.copy(path);
                }
            }
            //paste
            KeyCode::Char('v') => match file_manager.paste() {
                Err(_e) => return AppEvents::None,
                Ok(_) => file_manager.update(),
            },
            KeyCode::Char('x') => {
                let path = match self.selected_file_in_table(file_manager) {
                    None => return AppEvents::None,
                    Some(path) => path,
                };
                match file_manager.delete(&path) {
                    Err(_e) => todo!(),
                    Ok(_) => file_manager.update(),
                };
            }
            KeyCode::Char('h') => {
                file_manager.show_hidden = !file_manager.show_hidden;
                file_manager.update();
            }
            _ => {}
        }
        AppEvents::None
    }

    fn draw(&mut self, frame: &mut Frame, file_manager: &mut FileManager) {
        let title = Line::from("FILE EXPLORER").bold();
        let help_text = Line::from("Key Mappings:<m>");
        let block = Block::bordered()
            .title(title.left_aligned().bold())
            .border_set(border::THICK)
            .title_bottom(help_text.right_aligned().bold());
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
            if entry.metadata().unwrap().is_dir() {
                row = row.blue();
            }
            rows.push(row);
        }
        let widths = [Constraint::Percentage(20), Constraint::Percentage(20)];

        let table = Table::new(rows, widths)
            .block(block)
            .header(header)
            .cell_highlight_style(Style::new().green());

        frame.render_stateful_widget(table, frame.area(), &mut self.table_state);
    }
}
