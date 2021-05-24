pub struct Cursor {
    col: usize,
    row: usize,
}

impl Cursor {
    // constructor
    pub fn new() -> Self {
        Self { col: 0, row: 0 }
    }

    // accessors
    pub fn get_col(&self) -> usize {
        self.col.clone()
    }
    pub fn get_row(&self) -> usize {
        self.row.clone()
    }
    pub fn set_col(&mut self, value: usize) {
        self.col = value;
    }
    pub fn set_col_row(&mut self, col: usize, row: usize) {
        self.col = col;
        self.row = row;
    }
}
