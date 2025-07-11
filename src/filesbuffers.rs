use std::collections::HashMap;

pub type FileBuffer = Vec<char>;

#[derive(Default, Debug, Clone)]
pub struct FilesBuffers(HashMap<String, (FileBuffer, i32)>);

impl FilesBuffers {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn init_file_buffer(&mut self, key: String) {
        self.0.insert(key, (vec![], 0));
    }

    pub fn get(&self, key: String) -> (FileBuffer, i32) {
        self.0
            .get(&key)
            .cloned()
            .expect("Failed to get file buffer")
    }

    pub fn get_mut(&mut self, key: String) -> &mut (FileBuffer, i32) {
        self.0.get_mut(&key).expect("Failed to get file buffer")
    }
}
