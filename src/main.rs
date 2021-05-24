use std::env::args;
use ante::Editor;

fn main() {
    let mut editor = Editor::new(args().nth(1));
    editor.run();
}
