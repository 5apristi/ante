use crate::terminal::event::{Event, Key};
use crate::terminal::style::Color;
use crate::terminal::Terminal;
use crate::text_buffer::Buffer;
use crate::text_buffer::BufferStatus;
use std::cmp::min;
use std::path::PathBuf;

mod help_menu;
use help_menu::HELP_MENU_CONTENT;

mod cursor;
pub use cursor::Cursor;

const STATUS_BAR: &str = "ctrl + h: help menu";

// this structure represent the text editor.
pub struct Editor {
    // this flag will be turned on (true) if the user asks to quit
    will_quit_flag: bool,
    terminal: Terminal,
    text_buffer: Buffer,
    /* /!\ It's the cursor manipulating the text buffer,
    not the one drawn on the screen ! I didn't create another
    abstraction for the cursor drawn on the screen for simplicity's sake */
    cursor: Cursor,
    text_buffer_row_offset: usize,
    text_buffer_col_offset: usize,
}

impl Editor {
    // constructor
    /* returns a new instance of Editor struct,
    with or without argument (which in this case might be a file path, existing or not) */
    pub fn new(args: Option<String>) -> Self {
        Self {
            will_quit_flag: false,
            terminal: Terminal::new(),
            text_buffer: match args {
                Some(e) => Buffer::new_from_file(PathBuf::from(e)),
                None => Buffer::new_empty(),
            },
            cursor: Cursor::new(),
            text_buffer_row_offset: 0,
            text_buffer_col_offset: 0,
        }
    }

    // accessors
    fn current_row_position(&self) -> usize {
        self.cursor.get_row()
    }
    fn current_col_position(&self) -> usize {
        self.cursor.get_col()
    }

    // cursor
    fn move_cursor_up(&mut self) {
        if self.current_row_position() > 0 {
            if self.current_row_position() - self.text_buffer_row_offset == 0 {
                self.text_buffer_row_offset -= 1;
            }
            self.cursor.set_col_row(
                min(
                    self.current_col_position(),
                    self.text_buffer.get_lenght_of_row(self.current_row_position() - 1),
                ),
                self.current_row_position() - 1,
            );
            if self.current_col_position() < self.terminal.get_size_col() {
                self.text_buffer_col_offset = 0;
            } else {
                self.text_buffer_col_offset =
                    self.text_buffer.get_lenght_of_row(self.current_row_position()) - self.terminal.get_size_col();
            }
        }
    }
    fn move_cursor_down(&mut self) {
        if self.current_row_position() + 1 < self.text_buffer.get_lenght() {
            if self.current_row_position() - self.text_buffer_row_offset + 3 > self.terminal.get_size_row() {
                self.text_buffer_row_offset += 1;
            }
            self.cursor.set_col_row(
                min(
                    self.current_col_position(),
                    self.text_buffer.get_lenght_of_row(self.current_row_position() + 1),
                ),
                self.current_row_position() + 1,
            );
            if self.current_col_position() < self.terminal.get_size_col() {
                self.text_buffer_col_offset = 0;
            } else {
                self.text_buffer_col_offset =
                    self.text_buffer.get_lenght_of_row(self.current_row_position()) - self.terminal.get_size_col();
            }
        }
    }
    fn move_cursor_left(&mut self) {
        if self.current_col_position() - self.text_buffer_col_offset > 0 {
            self.cursor.set_col(self.current_col_position() - 1);
        } else {
            if self.text_buffer_col_offset > 0 {
                self.text_buffer_col_offset -= 1;
                self.cursor.set_col(self.current_col_position() - 1);
            }
        }
    }
    fn move_cursor_right(&mut self) {
        if self.current_col_position() < self.text_buffer.get_lenght_of_row(self.current_row_position()) {
            if self.current_col_position() - self.text_buffer_col_offset + 2 > self.terminal.get_size_col() {
                self.text_buffer_col_offset += 1;
            }
            self.cursor.set_col(self.current_col_position() + 1);
        }
    }

    // display
    fn draw(&mut self) {
        self.terminal.hide_cursor();
        let mut draw_cursor_row_position = 0;
        for i in self.text_buffer_row_offset..self.terminal.get_last_row() + self.text_buffer_row_offset {
            if i < self.text_buffer.get_lenght() {
                self.terminal.move_cursor_at(0, draw_cursor_row_position);
                draw_cursor_row_position += 1;
                self.terminal.clear_current_line();
                for j in self.text_buffer_col_offset..self.terminal.get_last_col() + self.text_buffer_col_offset {
                    if j < self.text_buffer.get_lenght_of_row(i) {
                        self.terminal.print(self.text_buffer.borrow_char_at(j, i));
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        self.draw_status_bar();
        self.terminal.move_cursor_at(
            self.current_col_position() - self.text_buffer_col_offset,
            self.current_row_position() - self.text_buffer_row_offset,
        );
        self.terminal.show_cursor();
        self.terminal.flush();
    }
    fn draw_status_bar(&mut self) {
        self.terminal.move_cursor_at(0, self.terminal.get_last_row());
        self.terminal.clear_current_line();
        if STATUS_BAR.len() + 7 < self.terminal.get_size_col() {
            self.terminal.print_text(STATUS_BAR, Color::Black, Color::White);
        }
        match self.text_buffer.get_path_as_str() {
            Some(s) => {
                if s.len() < self.terminal.get_size_col() {
                    self.terminal
                        .move_cursor_at(self.terminal.get_size_col() - s.len(), self.terminal.get_last_row());
                    let (fg_color, bg_color) = match self.text_buffer.get_status() {
                        BufferStatus::Saved => (Color::White, Color::Green),
                        BufferStatus::Unsaved => (Color::White, Color::Red),
                    };
                    self.terminal.print_text(s, fg_color, bg_color);
                }
            }
            None => {
                self.terminal
                    .move_cursor_at(self.terminal.get_size_col() - 7, self.terminal.get_last_row());
                self.terminal.print_text("unsaved", Color::White, Color::Red);
            }
        }
    }

    // events
    fn key_pressed(&mut self, key: Key) {
        match key {
            // edit buffer
            Key::Char(c) => {
                self.text_buffer
                    .insert_char(self.current_col_position(), self.current_row_position(), c);
                self.move_cursor_right(); // double vérif de current col pos, à revoir
            }
            Key::Backspace => self.backspace_key_pressed(),
            Key::Enter => self.enter_key_pressed(),
            // arrow keys
            Key::DownArrow => self.move_cursor_down(),
            Key::UpArrow => self.move_cursor_up(),
            Key::LeftArrow => self.move_cursor_left(),
            Key::RightArrow => self.move_cursor_right(),
            _ => (),
        }
    }
    fn backspace_key_pressed(&mut self) {
        if self.current_col_position() > 0 {
            self.text_buffer
                .delete_char(self.current_col_position() - 1, self.current_row_position());
            self.move_cursor_left();
        } else if self.current_row_position() != 0 {
            let previous_len_row_above = self.text_buffer.get_lenght_of_row(self.current_row_position() - 1);
            let mut data = self.text_buffer.remove_row_to_get_data(self.current_row_position());
            self.text_buffer.push_vec_to_row(self.current_row_position() - 1, &mut data);
            self.cursor.set_col_row(previous_len_row_above, self.current_row_position() - 1)
        }
    }
    fn enter_key_pressed(&mut self) {
        if self.current_col_position() == self.text_buffer.get_lenght_of_row(self.current_row_position()) {
            self.text_buffer.insert_row_at(self.current_row_position() + 1);
            self.move_cursor_down();
        } else {
            let vec = self
                .text_buffer
                .remove_row_from(self.current_col_position(), self.current_row_position());
            self.text_buffer.insert_row_at_with_vec(self.current_row_position() + 1, vec);
            self.move_cursor_down();
            self.cursor.set_col(0);
        }
    }
    fn key_pressed_with_ctrl(&mut self, key: Key) {
        match key {
            Key::Char('c') => self.will_quit_flag = true,
            Key::Char('s') => match self.text_buffer.get_path() {
                Some(_) => match self.text_buffer.save() {
                    BufferStatus::Saved => {}
                    BufferStatus::Unsaved => {}
                },
                None => {
                    let path = self.ask_user_for_path();
                    if let Some(p) = path {
                        match self.text_buffer.save_as(p) {
                            BufferStatus::Saved => {}
                            BufferStatus::Unsaved => {}
                        }
                    }
                }
            },
            Key::Char('o') => {
                self.open_new_file();
            },
            Key::Char('h') => self.open_help_menu(), // for some reasons ctrl + backspace open also the menu
            _ => (),
        }
    }
    fn open_new_file(&mut self) {
        if let Some(path) = self.ask_user_for_path() {
            self.text_buffer = Buffer::new_from_file(path);
        }
    }
    fn open_help_menu(&mut self) {
        self.terminal.clear_all();
        self.terminal.hide_cursor();

        self.terminal.move_cursor_at(0, 0);
        self.terminal.print_text("Help menu\n\n\r", Color::Black, Color::White);
        self.terminal.print(HELP_MENU_CONTENT);

        self.terminal.flush();
        loop {
            match self.terminal.read_event() {
                Event::CtrlKeyPressed(key) => match key {
                    Key::Char(c) if c == 'c' || c == 'h' => break,
                    _ => (),
                },
                _ => (),
            }
        }
        self.terminal.clear_all();
        self.terminal.flush();
    }

    fn ask_user_for_path(&mut self) -> Option<PathBuf> {
        self.terminal.move_cursor_at(0, self.terminal.get_last_row() - 1);
        self.terminal.clear_current_line();
        self.terminal.print_text("Path: ", Color::White, Color::Blue);
        self.terminal.flush();
        let mut path_buffer = String::new();
        loop {
            match self.terminal.read_event() {
                Event::KeyPressed(key) => match key {
                    Key::Char(c)
                        if path_buffer.len() + 7 < self.terminal.get_size_col()
                            && c != '<'
                            && c != '>'
                            && c != ':'
                            && c != '\"'
                            && c != '|'
                            && c != '?'
                            && c != '*' =>
                    {
                        path_buffer.push(c);
                    }
                    Key::Backspace if path_buffer.len() > 0 => {
                        path_buffer.pop();
                    }
                    Key::Enter => break,
                    Key::Esc => {
                        path_buffer.clear();
                        break;
                    }
                    _ => (),
                },
                Event::CtrlKeyPressed(key) => match key {
                    Key::Char(c) if c == 'c' => {
                        path_buffer.clear();
                        break;
                    }
                    _ => (),
                },
                _ => (),
            }
            self.terminal.move_cursor_at(0, self.terminal.get_size_row() - 2);
            self.terminal.clear_current_line();

            self.terminal.print_text("Path: ", Color::White, Color::Blue);
            self.terminal.print_text(&path_buffer, Color::White, Color::Blue);

            self.terminal.flush();
        }
        self.terminal.clear_current_line();
        if path_buffer.is_empty() {
            None
        } else {
            Some(PathBuf::from(path_buffer))
        }
    }

    // mainloop
    pub fn run(&mut self) {
        self.terminal.enter_alternate_screen();
        self.terminal.enable_raw_mode();
        loop {
            self.draw();
            if self.will_quit_flag == true {
                break;
            }
            match self.terminal.read_event() {
                Event::KeyPressed(key) => {
                    self.key_pressed(key);
                }
                Event::CtrlKeyPressed(key) => {
                    self.key_pressed_with_ctrl(key);
                }
                _ => (),
            }
        }
    }
}
