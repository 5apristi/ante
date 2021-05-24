#[derive(Clone, Copy, PartialEq)]
pub struct Size {
    cols: usize,
    rows: usize,
}

impl Size {
    // constructor
    pub fn new(size: (u16, u16)) -> Self {
        Self {
            cols: size.0 as usize,
            rows: size.1 as usize,
        }
    }

    // accessors
    pub fn get_cols(&self) -> usize {
        self.cols.clone()
    }
    pub fn get_rows(&self) -> usize {
        self.rows.clone()
    }
    pub fn set(&mut self, cols: usize, rows: usize) {
        self.cols = cols;
        self.rows = rows;
    }
}
