use std::io::Write;
use std::path::PathBuf;

mod row_buffer;
pub use row_buffer::RowBuffer;

#[derive(Clone, Copy)]
pub enum BufferStatus {
    Saved,
    Unsaved,
}

pub struct Buffer {
    data: Vec<RowBuffer>,
    lenght: usize,
    path: Option<PathBuf>,
    status: BufferStatus,
}

impl Buffer {
    // constructors
    pub fn new_from_file(file_path: PathBuf) -> Self {
        let data = std::fs::read_to_string(&file_path)
            .map(|string| {
                let mut vec: Vec<RowBuffer> = Vec::new();
                for line in string.lines() {
                    vec.push(RowBuffer::new_from_str(line.trim_end()));
                }
                vec
            })
            .unwrap_or(vec![RowBuffer::new_empty()]);
        Self {
            lenght: data.len(),
            data,
            path: Some(file_path),
            status: BufferStatus::Saved,
        }
    }
    pub fn new_empty() -> Self {
        Self {
            data: vec![RowBuffer::new_empty()],
            lenght: 1,
            path: None,
            status: BufferStatus::Unsaved,
        }
    }

    // to print
    pub fn borrow_row_at(&self, index: usize) -> &RowBuffer {
        &self.data[index]
    }
    pub fn borrow_char_at(&self, col: usize, row: usize) -> &char {
        self.data[row].borrow_char_at(col)
    }

    // accessors
    pub fn get_lenght(&self) -> usize {
        self.lenght
    }
    pub fn get_lenght_of_row(&self, index: usize) -> usize {
        self.data[index].get_lenght()
    }
    pub fn get_path(&self) -> Option<PathBuf> {
        self.path.clone()
    }
    pub fn get_path_as_str(&self) -> Option<&str> {
        match &self.path {
            Some(path) => match path.to_str() {
                Some(s) => Some(s),
                None => None,
            },
            None => None,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn row_is_empty(&self, index: usize) -> bool {
        self.data[index].is_empty()
    }
    // status accessors
    pub fn get_status(&self) -> BufferStatus {
        self.status.clone()
    }
    fn set_status(&mut self, new_status: BufferStatus) {
        self.status = new_status;
    }

    // write
    pub fn save(&mut self) -> BufferStatus {
        if let Some(path) = &self.path {
            let file = std::fs::File::create(path);
            match file {
                Ok(mut fp) => {
                    for i in 0..self.get_lenght() {
                        write!(fp, "{}\n", self.borrow_row_at(i)).unwrap();
                    }
                    self.set_status(BufferStatus::Saved);
                }
                Err(_) => {
                    self.set_status(BufferStatus::Unsaved);
                }
            }
        } else {
            self.set_status(BufferStatus::Unsaved);
        }
        self.get_status()
    }
    pub fn save_as(&mut self, path: PathBuf) -> BufferStatus {
        self.path = Some(path);
        if let BufferStatus::Unsaved = self.save() {
            self.path = None;
            self.set_status(BufferStatus::Unsaved);
        } else {
            self.set_status(BufferStatus::Saved);
        }
        self.get_status()
    }
    pub fn clear_path(&mut self) {
        self.path = None;
    }

    // manip buf
    pub fn insert_char(&mut self, col: usize, row: usize, c: char) {
        self.data[row].insert(col, c);
        self.set_status(BufferStatus::Unsaved);
    }
    pub fn insert_row_at(&mut self, index: usize) {
        self.data.insert(index, RowBuffer::new_empty());
        self.lenght += 1;
        self.set_status(BufferStatus::Unsaved);
    }
    pub fn insert_row_at_with_vec(&mut self, index: usize, vec: Vec<char>) {
        self.data.insert(index, RowBuffer::new_from_vec(vec));
        self.lenght += 1;
        self.set_status(BufferStatus::Unsaved);
    }
    pub fn delete_char(&mut self, col: usize, row: usize) {
        self.data[row].delete(col);
        self.set_status(BufferStatus::Unsaved);
    }
    pub fn remove_row_from(&mut self, col: usize, row: usize) -> Vec<char> {
        self.set_status(BufferStatus::Unsaved);
        self.data[row].remove_from(col)
    }
    pub fn remove_row_to_get_data(&mut self, index: usize) -> Vec<char> {
        self.set_status(BufferStatus::Unsaved);
        self.lenght -= 1;
        self.data.remove(index).unwrap_to_get_data()
    }
    pub fn push_vec_to_row(&mut self, index: usize, vec: &mut Vec<char>) {
        self.data[index].append_mb_vec_at_end(vec);
        self.set_status(BufferStatus::Unsaved);
    }
}
