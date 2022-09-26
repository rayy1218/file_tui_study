use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{FileType, Metadata, ReadDir};
use std::path::PathBuf;
use tui::widgets::{ListState};
use crate::app::state::AppState::Initialized;

#[derive(Clone)]
pub struct File {
    pub name: OsString,
    pub path: PathBuf,
    pub file_type: FileType,
    pub is_dir: bool,
    pub metadata: Metadata,
}

#[derive(Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn new(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        state.select(Some(0));

        StatefulList {
            state,
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn index(&self) -> usize {
        return self.state.selected().unwrap();
    }
}

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        current_list: StatefulList<File>,
        cursor: PathBuf,
        last_index: HashMap<String, usize>,
        display_log: bool,
        display_help: bool,
    },
}

impl AppState {
    pub fn initialized() -> Self {
        let mut items:Vec<File> = Vec::new();

        let paths = std::fs::read_dir("./").unwrap();
        for path in paths {
            let entry = path.unwrap();

            items.push(File {
                name: entry.file_name(),
                path: entry.path(),
                file_type: entry.file_type().unwrap(),
                is_dir: entry.path().is_dir(),
                metadata: entry.metadata().unwrap(),
            });
        }

        let current_list = StatefulList::new(items);
        let cursor = std::env::current_dir().unwrap();
        let mut last_index = HashMap::new();
        let display_log = false;
        let display_help = false;

        last_index.insert(
            cursor.to_str().unwrap().to_string(),
            current_list.state.selected().unwrap()
        );

        let mut ret = Initialized {
            current_list,
            cursor,
            last_index,
            display_log,
            display_help,
        };

        ret.sort_dir_items();

        ret
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn read_parent(&mut self) {
        match self {
            Initialized { cursor, last_index, current_list, .. } => {
                match cursor.parent() {
                    Some(path) => {
                        let mut dir = path.read_dir().unwrap();
                        let ori_dir_path = cursor.clone();

                        // record which item user left the cursor before transverse
                        last_index.insert(
                            cursor.to_str().unwrap().to_string(),
                            current_list.state.selected().unwrap()
                        );

                        *cursor = path.to_path_buf();
                        self.set_list(&mut dir);
                        self.adjust_parent_cursor(ori_dir_path.file_name().unwrap().to_str().unwrap());
                    }

                    _ => {}
                }
            },
            _ => {}
        }
    }

    fn adjust_parent_cursor(&mut self, dir_name: &str) {
        match self {
            Initialized { current_list,  .. } => {
                let mut index:usize = 0;
                for item in &current_list.items {
                    if item.name == dir_name {
                        current_list.state.select(Some(index));
                        return;
                    }

                    index += 1;
                }

                current_list.state.select(Some(0));
            },
            _ => {}
        }
    }

    pub fn read_dir(&mut self) {
        match self {
            Initialized { cursor, current_list, last_index, .. } => {
                let item:&File = current_list.items.get(current_list.index()).unwrap();

                if !item.is_dir {
                    return;
                }

                // record which item user left the cursor before transverse
                last_index.insert(
                    cursor.to_str().unwrap().to_string(),
                    current_list.state.selected().unwrap()
                );

                let selected_dir = &item.name;
                cursor.push(selected_dir.to_str().unwrap());

                let mut selected_dir_path = std::fs::read_dir(cursor.to_str().unwrap()).unwrap();

                self.set_list(&mut selected_dir_path);

                self.adjust_dir_cursor();
            },
            _ => {}
        }
    }

    fn adjust_dir_cursor(&mut self) {
        match self {
            Initialized { cursor, current_list, last_index, .. } => {
                match last_index.get(cursor.to_str().unwrap()) {
                    None => {
                        current_list.state.select(Some(0));
                    }
                    Some(value) => {
                        if *value >= current_list.items.len() {
                            current_list.state.select(Some(0));
                        }

                        current_list.state.select(Some(*value));
                    }
                }
            },
            _ => {}
        }
    }

    fn set_list(&mut self, dir:&mut ReadDir) {
        if let Initialized { current_list, .. } = self {
            let mut items:Vec<File> = Vec::new();

            for path in dir {
                let entry = path.unwrap();

                items.push(File {
                    name: entry.file_name(),
                    path: entry.path(),
                    file_type: entry.file_type().unwrap(),
                    is_dir: entry.path().is_dir(),
                    metadata: entry.metadata().unwrap(),
                });
            }

            current_list.items = items;

            self.sort_dir_items();
        }
    }

    pub fn sort_dir_items(&mut self) {
        if let Initialized { current_list, .. } = self {
            let mut dir_items:Vec<File>= Vec::new();
            let mut file_items:Vec<File> = Vec::new();

            for item in &current_list.items {
                if item.is_dir {
                    dir_items.push(item.clone());
                }
                else {
                    file_items.push(item.clone());
                }
            }

            dir_items.append(&mut file_items);

            current_list.items = dir_items;
        }
    }

    pub fn cursor(&self) -> Option<&PathBuf> {
        if let Initialized { cursor, .. } = self {
            Some(cursor)
        }
        else {
            None
        }
    }

    pub fn current_list(&self) -> Option<&StatefulList<File>> {
        if let Initialized { current_list, .. } = self {
            Some(current_list)
        }
        else {
            None
        }
    }

    pub fn display_log(&self) -> Option<&bool> {
        if let Initialized { display_log, .. } = self {
            Some(display_log)
        }
        else {
            None
        }
    }

    pub fn toggle_log(&mut self) {
        if let Initialized { display_log, .. } = self {
            *display_log = !*display_log;
        }
    }

    pub fn display_help(&self) -> Option<&bool> {
        if let Initialized { display_help, .. } = self {
            Some(display_help)
        }
        else {
            None
        }
    }

    pub fn toggle_help(&mut self) {
        if let Initialized { display_help, .. } = self {
            *display_help = !*display_help;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}