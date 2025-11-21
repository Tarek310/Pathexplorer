use std::cmp::Ordering;
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
    copy_buffer: Vec<PathBuf>,
}

impl FileManager {
    /// changes the content of the FileManager to the Files of the new path
    /// This method might panic!
    pub fn change_dir(&mut self, path_buf: PathBuf) {
        let p: &Path = PathBuf::as_path(&path_buf);
        let res = std::env::set_current_dir(p);
        if res.is_err() {
            return;
        }

        let entry_iter = fs::read_dir(Path::new(".")).unwrap();
        self.files.clear();
        self.num_files = 0;
        for entry_res in entry_iter {
            let entry = entry_res.unwrap();
            if self.show_hidden || !entry.file_name().to_str().unwrap().starts_with(".") {
                self.files.push(entry);
                self.num_files += 1;
            }
        }
        self.sort(self.curr_sort);
    }

    ///update file_manager for current directory!
    pub fn update(&mut self) {
        self.change_dir(PathBuf::from("."));
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
            copy_buffer: Vec::new(),
        };
        fm.change_dir(PathBuf::from("."));
        fm
    }

    pub fn get_entries(&self) -> &Vec<DirEntry> {
        &self.files
    }

    pub fn sort(&mut self, sort_mode: Sorting) {
        match sort_mode {
            //TODO!!!!!!!
            Sorting::SortedBySizeDescending => self.files.sort_by(|b, a| {
                a.metadata()
                    .unwrap()
                    .len()
                    .cmp(&b.metadata().unwrap().len())
            }),
            Sorting::SortedBySizeAscending => self.files.sort_by(|a, b| {
                a.metadata()
                    .unwrap()
                    .len()
                    .cmp(&b.metadata().unwrap().len())
            }),
            Sorting::SortedByNameDescending => self.files.sort_by(|b, a| {
                a.file_name()
                    .into_string()
                    .unwrap()
                    .cmp(&b.file_name().into_string().unwrap())
            }),
            Sorting::SortedByNameAscending => self.files.sort_by(|a, b: &DirEntry| {
                a.file_name()
                    .into_string()
                    .unwrap()
                    .cmp(&b.file_name().into_string().unwrap())
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
        if entry1.file_type().unwrap().is_dir() {
            Ordering::Less
        } else if entry2.file_type().unwrap().is_dir() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    ///When copying multiple files, use this function
    pub fn add_copy(&mut self, pb: PathBuf) {
        let full_path = std::path::absolute(pb);
        match full_path {
            Ok(full_path) => {
                self.copy_buffer.push(full_path);
            }
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }

    pub fn clear_copy(&mut self) {
        self.copy_buffer.clear();
    }

    pub fn copy(&mut self, pb: PathBuf) {
        self.clear_copy();
        self.add_copy(pb);
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
        for src in &self.copy_buffer {
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
}
