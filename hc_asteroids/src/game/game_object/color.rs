pub enum ColorName {
    Black,
    Red,
    Fuchsia,
    Blue,
    Cyan,
    Green,
    Yellow,
    White,
}

pub struct Color {
    pub r: bool,
    pub g: bool,
    pub b: bool,
}

impl Color {
    pub fn new(name: ColorName) -> Color {
        let value = name as u8;
        Color {
            r: (value & 1) > 0,
            g: (value & 2) > 0,
            b: (value & 4) > 0,
        }
    }
}