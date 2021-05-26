use ante::Editor;
use std::env::args;
use std::process::exit;

fn main() {
    let path_arg = args().nth(1);
    if let Some(s) = &path_arg {
        for character in s.chars() {
            match character {
                '<' | '>' | ':' | '\"' | '/' | '\\' | '|' | '?' | '*' => {
                    println!("Invalid character: {}", character);
                    exit(0);
                }
                _ => (),
            }
        }
    }
    let mut editor = Editor::new(path_arg);
    editor.run();
}
