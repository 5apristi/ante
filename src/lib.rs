/* external dependencies */
// provides the backend for terminal
extern crate crossterm;


/* modules */
// contains the Editor struct which represent the text editor itself
pub mod editor;
// interface to manipulate terminal (using Terminal struct),
// using Crossterm as backend
pub mod terminal;
// the text buffer
pub mod text_buffer;


/* re-exporting Editor to make it visible from main.rs */
pub use editor::Editor;

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