use crossterm::event::KeyCode;

pub enum Event {
    KeyPressed(Key),
    CtrlKeyPressed(Key),
    WindowResized(usize, usize),
    Unknown,
}

pub enum Key {
    Char(char),
    Backspace,
    Enter,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    Tab,
    Esc,
}

pub fn single_key_pressed(key_code: KeyCode) -> Event {
    match key_code {
        KeyCode::Char(c) => Event::KeyPressed(Key::Char(c)),
        KeyCode::Backspace => Event::KeyPressed(Key::Backspace),
        KeyCode::Enter => Event::KeyPressed(Key::Enter),
        KeyCode::Left => Event::KeyPressed(Key::LeftArrow),
        KeyCode::Right => Event::KeyPressed(Key::RightArrow),
        KeyCode::Up => Event::KeyPressed(Key::UpArrow),
        KeyCode::Down => Event::KeyPressed(Key::DownArrow),
        KeyCode::Tab => Event::KeyPressed(Key::Tab),
        KeyCode::Esc => Event::KeyPressed(Key::Esc),
        _ => Event::Unknown,
    }
}
pub fn key_pressed_with_control(key_code: KeyCode) -> Event {
    match key_code {
        KeyCode::Char(c) => Event::CtrlKeyPressed(Key::Char(c)),
        KeyCode::Backspace => Event::CtrlKeyPressed(Key::Backspace),
        KeyCode::Enter => Event::CtrlKeyPressed(Key::Enter),
        KeyCode::Left => Event::CtrlKeyPressed(Key::LeftArrow),
        KeyCode::Right => Event::CtrlKeyPressed(Key::RightArrow),
        KeyCode::Up => Event::CtrlKeyPressed(Key::UpArrow),
        KeyCode::Down => Event::CtrlKeyPressed(Key::DownArrow),
        KeyCode::Tab => Event::CtrlKeyPressed(Key::Tab),
        KeyCode::Esc => Event::CtrlKeyPressed(Key::Esc),
        _ => Event::Unknown,
    }
}
