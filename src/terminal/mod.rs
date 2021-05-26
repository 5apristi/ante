use crossterm::event::Event as CrosstermEvent;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::{terminal, Command, ExecutableCommand, QueueableCommand};
use event::{key_pressed_with_control, single_key_pressed, Event};
use std::io::{stdout, Stdout, Write};

pub use size::Size;

pub mod event;
mod size;

pub struct Terminal {
    output: Stdout,
    size: Size,
}

impl Terminal {
    // constructor
    pub fn new() -> Self {
        Self {
            output: stdout(),
            size: Size::new(terminal::size().unwrap()),
        }
    }

    // alternate screen methods
    pub fn enter_alternate_screen(&mut self) {
        self.queue(terminal::EnterAlternateScreen);
    }
    pub fn leave_alternate_screen(&mut self) {
        self.queue(terminal::LeaveAlternateScreen);
    }

    // raw mode
    pub fn enable_raw_mode(&self) {
        terminal::enable_raw_mode().unwrap();
    }
    pub fn disable_raw_mode(&self) {
        terminal::disable_raw_mode().unwrap();
    }

    // display
    pub fn print(&mut self, impl_display: impl std::fmt::Display) {
        write!(self.output, "{}", impl_display).unwrap();
    }
    pub fn clear_all(&mut self) {
        self.queue(crossterm::terminal::Clear(crossterm::terminal::ClearType::All));
    }
    pub fn clear_current_line(&mut self) {
        self.queue(crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine));
    }

    // command
    pub fn queue(&mut self, command: impl Command) {
        self.output.queue(command).unwrap();
    }
    pub fn execute(&mut self, command: impl Command) {
        self.output.execute(command).unwrap();
    }
    pub fn flush(&mut self) {
        self.output.flush().unwrap();
    }

    // cursor
    pub fn show_cursor(&mut self) {
        self.queue(crossterm::cursor::Show);
    }
    pub fn hide_cursor(&mut self) {
        self.queue(crossterm::cursor::Show);
    }
    pub fn move_cursor_down(&mut self) {
        if crossterm::cursor::position().unwrap().0 < u16::MAX {
            self.queue(crossterm::cursor::MoveDown(1));
        }
    }
    pub fn move_cursor_up(&mut self) {
        if crossterm::cursor::position().unwrap().0 > u16::MIN {
            self.queue(crossterm::cursor::MoveUp(1));
        }
    }
    pub fn move_cursor_left(&mut self) {
        if crossterm::cursor::position().unwrap().1 > u16::MIN {
            self.queue(crossterm::cursor::MoveLeft(1));
        }
    }
    pub fn move_cursor_right(&mut self) {
        if crossterm::cursor::position().unwrap().1 < u16::MAX {
            self.queue(crossterm::cursor::MoveRight(1));
        }
    }
    pub fn move_cursor_at(&mut self, col: usize, row: usize) {
        if col <= u16::MAX as usize && col >= u16::MIN as usize && row <= u16::MAX as usize && row >= u16::MIN as usize {
            self.queue(crossterm::cursor::MoveTo(col as u16, row as u16));
        }
    }

    // accessors
    pub fn get_size_col(&self) -> usize {
        self.size.get_cols()
    }
    pub fn get_size_row(&self) -> usize {
        self.size.get_rows()
    }
    pub fn get_size(&self) -> Size {
        self.size.clone()
    }

    // events
    pub fn read_event(&mut self) -> Event {
        let event = crossterm::event::read().unwrap();
        match event {
            CrosstermEvent::Key(key_event)
                if ((key_event.modifiers == KeyModifiers::NONE) || (key_event.modifiers == KeyModifiers::SHIFT)) =>
            {
                single_key_pressed(key_event.code)
            }
            CrosstermEvent::Key(key_event) if key_event.modifiers == KeyModifiers::CONTROL && key_event.code != KeyCode::Null => {
                key_pressed_with_control(key_event.code)
            }
            CrosstermEvent::Resize(cols, rows) => {
                self.update_size(cols as usize, rows as usize);
                self.clear_all();
                Event::WindowResized(cols as usize, rows as usize)
            }
            _ => Event::Unknown,
        }
    }
    fn update_size(&mut self, cols: usize, rows: usize) {
        self.size.set(cols, rows);
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.queue(terminal::LeaveAlternateScreen);
        self.queue(crossterm::cursor::Show);
        self.flush();
        terminal::disable_raw_mode().unwrap();
    }
}
