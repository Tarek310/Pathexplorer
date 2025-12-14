use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::{DirEntry, create_dir};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Copy, Clone)]
pub enum Sorting {
    Unsorted,
    SortedBySizeDescending,
    SortedBySizeAscending,
    SortedByNameDescending,
    SortedByNameAscending,
}

pub enum SortDir {
    Unsorted,
    Start,
    End,
}

pub struct FileManager {
    files: Vec<DirEntry>,
    pub num_files: usize,
    curr_sort: Sorting,
    pub show_hidden: bool,
    pub dir_sorting: SortDir,
    selection: HashSet<PathBuf>,

    error_queue: Vec<io::Error>,
}

impl FileManager {
    /// changes the content of the FileManager to the Files of the new path
    /// This method might panic!
    pub fn change_dir(&mut self, path_buf: PathBuf) -> io::Result<()> {
        let p: &Path = PathBuf::as_path(&path_buf);

        let entry_iter = fs::read_dir(p)?;
        std::env::set_current_dir(p)?;

        self.files.clear();
        self.num_files = 0;
        for entry in entry_iter.flatten() {
            if self.show_hidden
                || !entry
                    .file_name()
                    .to_str()
                    .unwrap_or_default()
                    .starts_with(".")
            {
                self.files.push(entry);
                self.num_files += 1;
            }
        }
        self.sort(self.curr_sort);
        Ok(())
    }

    pub fn current_dir(&self) -> io::Result<PathBuf> {
        std::env::current_dir()
    }

    ///update file_manager for current directory!
    pub fn update(&mut self) {
        let _ = self.change_dir(PathBuf::from("."));
    }

    ///creates and initializes a FileManager-struct
    ///Calls change_dir on the CWD!
    pub fn new() -> FileManager {
        let mut fm: FileManager = FileManager {
            files: Vec::new(),
            num_files: 0,
            curr_sort: Sorting::Unsorted,
            show_hidden: false,
            dir_sorting: SortDir::Unsorted,
            selection: HashSet::new(),
            error_queue: Vec::new(),
        };
        let _ = fm.change_dir(PathBuf::from("."));
        fm
    }

    pub fn get_entries(&self) -> &Vec<DirEntry> {
        &self.files
    }

    pub fn sort(&mut self, sort_mode: Sorting) {
        match sort_mode {
            //TODO!!!!!!!
            Sorting::SortedBySizeDescending => self.files.sort_by(|b, a| {
                let a_meta = a.metadata().map(|m| m.len()).unwrap_or_default();
                let b_meta = b.metadata().map(|m| m.len()).unwrap_or_default();
                a_meta.cmp(&b_meta)
            }),
            Sorting::SortedBySizeAscending => self.files.sort_by(|a, b| {
                let a_meta = a.metadata().map(|m| m.len()).unwrap_or_default();
                let b_meta = b.metadata().map(|m| m.len()).unwrap_or_default();
                a_meta.cmp(&b_meta)
            }),
            Sorting::SortedByNameDescending => self.files.sort_by(|b, a| {
                let a_string = a.file_name().into_string().unwrap_or_default();
                let b_string = b.file_name().into_string().unwrap_or_default();
                a_string.cmp(&b_string)
            }),
            Sorting::SortedByNameAscending => self.files.sort_by(|a, b: &DirEntry| {
                let a_string = a.file_name().into_string().unwrap_or_default();
                let b_string = b.file_name().into_string().unwrap_or_default();
                a_string.cmp(&b_string)
            }),
            _ => {}
        };
        self.curr_sort = sort_mode;
        match self.dir_sorting {
            SortDir::Start => {
                self.files
                    .sort_by(|a, b| FileManager::sort_dir_to_start(a, b));
            }
            SortDir::End => {
                self.files
                    .sort_by(|b, a| FileManager::sort_dir_to_start(a, b));
            }
            SortDir::Unsorted => {}
        }
    }

    fn sort_dir_to_start(entry1: &DirEntry, entry2: &DirEntry) -> Ordering {
        if entry1.file_type().map(|e| e.is_dir()).unwrap_or(false) {
            Ordering::Less
        } else if entry2.file_type().map(|e| e.is_dir()).unwrap_or(false) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    ///add file to selection
    pub fn add_to_selection(&mut self, pb: PathBuf) {
        match std::path::absolute(pb) {
            Ok(full_path) => {
                self.selection.insert(full_path);
            }
            Err(e) => {
                self.push_error(e);
            }
        }
    }

    ///remove file from selection
    pub fn remove_from_selection(&mut self, pb: PathBuf) {
        match std::path::absolute(pb) {
            Ok(full_path) => {
                self.selection.remove(&full_path);
            }
            Err(e) => {
                self.push_error(e);
            }
        }
    }

    pub fn is_selected(&self, path: &PathBuf) -> bool {
        match std::path::absolute(path) {
            Ok(full_path) => self.selection.contains(&full_path),
            Err(_) => false,
        }
    }

    pub fn delete_selection(&mut self) {
        let selection_clone = self.selection.clone();
        for dest in selection_clone {
            if let Err(e) = self.delete(&dest) {
                self.push_error(e);
            }
        }
        self.selection.clear();
        self.update();
    }

    pub fn delete(&mut self, dest: &PathBuf) -> io::Result<()> {
        if dest.is_dir() {
            fs::remove_dir_all(dest)?;
            Ok(())
        } else if dest.is_file() || dest.is_symlink() {
            fs::remove_file(dest)?;
            Ok(())
        } else {
            panic!();
        }
    }

    ///paste the content of copy_buffer into the current directory!
    ///deep-copies directories
    pub fn paste(&mut self) -> io::Result<()> {
        let current_dir = std::path::absolute(PathBuf::from("."))?;

        for src in &self.selection {
            //check if src dir gets copied into itself
            if src.is_dir() && current_dir.starts_with(src) {
                continue;
            }

            if src.is_file() {
                fs::copy(
                    src,
                    PathBuf::from(src.file_name().unwrap().to_str().unwrap()),
                )?;
            }
            //copying the directory and recursively copy it's content into the new directory
            else if src.is_dir() {
                let src_folder_name = match src.file_name() {
                    None => todo!(),
                    Some(name) => match name.to_str() {
                        None => todo!(),
                        Some(name_str) => name_str,
                    },
                };

                let dest_folder = PathBuf::from(src_folder_name);
                create_dir(&dest_folder)?;

                let mut stack: Vec<PathBuf> = Vec::new(); //contains relative paths within the source directory
                stack.push(PathBuf::from(".")); //start with the root of the source directory

                loop {
                    let current_relative_path = match stack.pop() {
                        None => break, //stack is empty
                        Some(path) => path,
                    };

                    let current_src_path = src.join(&current_relative_path);
                    let entry_iter = fs::read_dir(&current_src_path)?;

                    for entry_res in entry_iter {
                        let entry = match entry_res {
                            Err(_e) => continue,
                            Ok(entry) => entry,
                        };
                        let file_type = match entry.file_type() {
                            Err(_e) => continue,
                            Ok(file_type) => file_type,
                        };

                        let relative_entry_path = current_relative_path.join(entry.file_name());
                        let src_entry = src.join(&relative_entry_path);
                        let dest_entry = dest_folder.join(&relative_entry_path);

                        if file_type.is_dir() {
                            create_dir(&dest_entry)?;
                            stack.push(relative_entry_path);
                        } else if file_type.is_file() {
                            fs::copy(src_entry, dest_entry)?;
                        } else if file_type.is_symlink() {
                            let link_target = fs::read_link(&src_entry)?;
                            #[cfg(unix)]
                            std::os::unix::fs::symlink(link_target, dest_entry)?;
                            #[cfg(windows)]
                            {
                                if src_entry.is_dir() {
                                    std::os::windows::fs::symlink_dir(link_target, dest_entry)?;
                                } else {
                                    std::os::windows::fs::symlink_file(link_target, dest_entry)?;
                                }
                            }
                        } else {
                            panic!();
                        }
                    }
                }
            }
        }
        self.update();
        Ok(())
    }

    pub fn get_entry_at_index(&self, index: usize) -> Result<&DirEntry, Error> {
        let entry = self.get_entries().get(index);
        match entry {
            Some(entry) => Ok(entry),
            None => Err(Error::new(ErrorKind::NotFound, "wrong index")),
        }
    }

    pub fn create_file(&mut self, path: PathBuf) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::File::create(path)?;
        self.update();
        Ok(())
    }

    pub fn create_folder(&mut self, path: PathBuf) -> io::Result<()> {
        fs::create_dir_all(path)?;
        self.update();
        Ok(())
    }

    pub fn take_errors(&mut self) -> Vec<io::Error> {
        std::mem::take(&mut self.error_queue)
    }

    fn push_error(&mut self, error: io::Error) {
        self.error_queue.push(error);
    }
}
