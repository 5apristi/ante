/*** --------------------- ***/
/*** external dependencies ***/
/*** --------------------- ***/

// Provides the backend for terminal.
extern crate crossterm;

/*** ------- ***/
/*** modules ***/
/*** ------- ***/

// It Contains the Editor struct which represent the text editor itself.
pub mod editor;

// Interface to manipulate terminal (using Terminal struct),
// using Crossterm as backend.
pub mod terminal;

// It contains the TextBuffer struct which stores into the heap text data encoded in utf-8.
pub mod text_buffer;

// Config module: it contains default configuration for ante and parses user's configuration file. 
pub mod config;

/*** ---------- ***/
/*** Re-exports ***/
/*** ---------- ***/

// It makes Editor visible from main.rs.
pub use editor::Editor;

/*** ----- ***/
/*** Tests ***/
/*** ----- ***/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_editor() {
        let _editor = editor::Editor::new(Option::None);
    }

    #[test]
    fn init_text_buffer() {
        let _buffer = text_buffer::Buffer::new_empty();
    }

    #[test]
    fn init_terminal() {
        let _terminal = terminal::Terminal::new();
    }
}
