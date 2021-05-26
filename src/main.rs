use ante::Editor;
use std::env::args;

fn main() {
    let mut editor = Editor::new(args().nth(1));
    editor.run();
}
