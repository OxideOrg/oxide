use std::collections::HashMap;

use ratatui::layout::Position;

#[derive(Default, Debug, Clone)]
pub struct FilesBuffers {
    files: HashMap<String, FileBuffer>,
}

#[derive(Default, Debug, Clone)]
pub struct FileBuffer {
    pub file: Vec<Vec<char>>,
    pub current_line: u16,
    pub current_column: u16,
    pub lines_number: u16,
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

    pub fn init_file_buffer(&mut self, key: String) {
        self.files.insert(
            key,
            FileBuffer {
                file: vec![vec![]],
                current_line: 0,
                current_column: 0,
                lines_number: 0,
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
            x: self.current_column,
            y: self.current_line,
        }
    }

    pub fn create_line(&mut self) {
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
}
