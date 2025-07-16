use std::{collections::HashMap, fs::File, io::Read};

use ratatui::layout::Position;

use crate::ui::LINE_NUMBERS_WIDTH;

pub enum Move {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Default, Debug, Clone)]
pub struct FilesBuffers {
    pub files: HashMap<String, FileBuffer>,
}

#[derive(Default, Debug, Clone)]
pub struct FileBuffer {
    pub file: Vec<Vec<char>>,
    pub current_line: u16,
    pub current_column: u16,
    pub lines_number: u16,
    pub scroll_y: u16,
}

impl FilesBuffers {
    pub fn new() -> Self {
        FilesBuffers {
            files: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn init_file_buffer(&mut self, file_path: String) {
        let mut buffer = vec![];
        let mut lines_number = 0;
        if let Ok(mut file) = File::open(&file_path) {
            let mut file_content = String::new();
            file.read_to_string(&mut file_content)
                .expect("Failed to read file buffer");
            if file_content.lines().count() == 0 {
                buffer.push(vec![]);
            }
            for line in file_content.lines() {
                let mut buffer_line = vec![];
                for c in line.chars() {
                    buffer_line.push(c);
                }
                buffer.push(buffer_line);
                lines_number += 1;
            }
        } else {
            buffer.push(vec![]);
        }
        self.files.insert(
            file_path,
            FileBuffer {
                file: buffer,
                current_line: 0,
                current_column: 0,
                lines_number,
                scroll_y: 0,
            },
        );
    }

    pub fn get(&self, key: String) -> FileBuffer {
        self.files
            .get(&key)
            .cloned()
            .expect("Failed to get file buffer")
    }

    pub fn get_mut(&mut self, key: String) -> &mut FileBuffer {
        self.files.get_mut(&key).expect("Failed to get file buffer")
    }
}

impl FileBuffer {
    pub fn get_mut(&mut self) -> &mut Vec<char> {
        self.file
            .get_mut(self.current_line as usize)
            .expect("Failed to get file buffer for given line")
    }

    pub fn to_cursor_position(&self) -> Position {
        Position {
            x: self.current_column + LINE_NUMBERS_WIDTH,
            y: self.current_line,
        }
    }

    pub fn create_line(&mut self) {
        //TODO: Create line should create line at current position and not only after current line
        self.file.insert(self.current_line as usize + 1, vec![]);
        self.current_line += 1;
        self.current_column = 0;
        self.lines_number += 1;
    }

    pub fn delete_line(&mut self) {
        let current_line = self.current_line as usize;
        self.file.remove(current_line);
        self.current_line -= 1;
        self.current_column = self.get_mut().len() as u16;
        self.lines_number -= 1;
    }

    pub fn insert_char(&mut self, input: char) {
        let current_column = self.current_column as usize;
        let buffer = self.get_mut();
        buffer.insert(current_column, input);
        self.current_column += 1;
    }

    pub fn delete_previous_position(&mut self) {
        if self.current_column > 0 {
            let current_column = self.current_column as usize - 1;
            let buffer = self.get_mut();
            buffer.remove(current_column);
            self.current_column -= 1;
        } else if self.current_line > 0 {
            self.delete_line();
        }
    }

    pub fn move_cursor(&mut self, move_option: Move) {
        let columns_number = self.get_mut().iter().count() as u16;
        match move_option {
            Move::Left => {
                if self.current_column > 0 {
                    self.current_column -= 1
                }
            }
            Move::Up => {
                if self.current_line > 0 {
                    self.current_line -= 1
                }
            }
            Move::Right => {
                if self.current_column < columns_number {
                    self.current_column += 1
                }
            }
            Move::Down => {
                if self.current_line + 1 < self.lines_number {
                    self.current_line += 1
                }
            }
        };
    }

    pub fn move_to_next_word(&mut self) {
        let columns_number = self.get_mut().iter().count() as u16;
        let mut index = self.current_column;
        {
            let line = self.get_mut();
            let mut is_parsing_word = false;
            while index < columns_number {
                let Some(c) = line.get(index as usize) else {
                    return;
                };
                if is_parsing_word && *c == ' ' {
                    break;
                }
                if *c != ' ' {
                    is_parsing_word = true;
                }
                index += 1;
            }
        }
        self.current_column = index;
    }

    pub fn move_to_previous_word(&mut self) {
        if self.current_column < 1 {
            return;
        };
        let mut index = self.current_column - 1;
        {
            let line = self.get_mut();
            let mut is_parsing_word = false;
            while index > 0 {
                let Some(c) = line.get(index as usize) else {
                    return;
                };
                if is_parsing_word && *c == ' ' {
                    break;
                }
                if *c != ' ' {
                    is_parsing_word = true;
                }
                index -= 1;
            }
        }
        self.current_column = index;
    }
}
