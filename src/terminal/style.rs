use crossterm::style::Color as CrosstermColor;

pub enum Color {
    Black,
    Red,
    Green,
    Blue,
    White,
    Yellow,
    Grey,
    Rgb { red: u8, green: u8, blue: u8 },
}

pub fn convert_crossterm_color_enum(color: Color) -> CrosstermColor {
    match color {
        Color::Black => CrosstermColor::Black,
        Color::Red => CrosstermColor::Red,
        Color::Green => CrosstermColor::Green,
        Color::Blue => CrosstermColor::Blue,
        Color::White => CrosstermColor::White,
        Color::Yellow => CrosstermColor::Yellow,
        Color::Grey => CrosstermColor::Grey,
        Color::Rgb { red, green, blue } => CrosstermColor::Rgb { r: red, g: green, b: blue },
    }
}
