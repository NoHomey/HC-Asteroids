pub enum ColorName {
    Black,
    Blue,
    Red,
    Purple,
    Green,
    Cyan,
    Yellow,
    White,
}

pub fn number_to_color(num: u8) -> ColorName {
    match num {
        0 => ColorName::Black,
        1 => ColorName::Blue,
        2 => ColorName::Red,
        3 => ColorName::Purple,
        4 => ColorName::Green,
        5 => ColorName::Cyan,
        6 => ColorName::Yellow,
        _ => ColorName::White,
    }
}

pub struct Color {
    b: bool,
    r: bool,
    g: bool,
    pub color: u8,
}

fn trf(val: bool) -> u16 {
    if val { 4095 } else { 0 }
}

impl Color {
    pub fn new(name: ColorName) -> Color {
        let value = name as u8;
        Color {
            b: (value & 1) > 0,
            r: (value & 2) > 0,
            g: (value & 4) > 0,
            color: value,
        }
    }
    
    pub fn to_array(&self) -> [u16; 3] {
       [trf(self.b), trf(self.r), trf(self.g)]
    }
}