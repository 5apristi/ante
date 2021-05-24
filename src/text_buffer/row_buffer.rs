pub struct RowBuffer {
    data: Vec<char>,
    lenght: usize,
}

impl RowBuffer {
    // constructors
    pub fn new_from_str(data: &str) -> Self {
        Self {
            data: data.chars().collect::<Vec<char>>(),
            lenght: data.chars().count(),
        }
    }
    pub fn new_from_slice_char(slice: &[char]) -> Self {
        Self {
            lenght: slice.len(),
            data: {
                let mut temp = Vec::new();
                for c in slice.iter() {
                    temp.push(*c);
                }
                temp
            },
        }
    }
    pub fn new_from_vec(vec: Vec<char>) -> Self {
        Self { lenght: vec.len(), data: vec }
    }
    pub fn new_empty() -> Self {
        Self { data: Vec::new(), lenght: 0 }
    }

    // accessors
    pub fn get_data(&self) -> &[char] {
        self.data.as_slice()
    }
    pub fn get_lenght(&self) -> usize {
        self.lenght.clone()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn borrow_char_at(&self, index: usize) -> &char {
        &self.data[index]
    }

    // manip buf
    pub fn insert(&mut self, index: usize, c: char) {
        if index < self.lenght {
            self.data.insert(index, c);
        } else {
            self.data.push(c);
        }
        self.lenght += 1;
    }
    pub fn delete(&mut self, index: usize) {
        if index < self.lenght {
            self.data.remove(index);
            self.lenght -= 1;
        } else {
            panic!("index >= self.lenght");
        }
    }
    pub fn remove_from(&mut self, index: usize) -> Vec<char> {
        let mut temp = Vec::new();
        for _i in index..self.lenght {
            temp.push(self.data.remove(index));
        }
        self.lenght -= self.lenght - index;
        temp
    }
    pub fn append_mb_vec_at_end(&mut self, vec: &mut Vec<char>) {
        self.lenght += vec.len();
        self.data.append(vec);
    }
    pub fn unwrap_to_get_data(self) -> Vec<char> {
        self.data
    }
}

impl std::fmt::Display for &RowBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut to_print = String::new();
        for c in &self.data {
            to_print.push(*c);
        }
        write!(f, "{}", to_print)
    }
}
