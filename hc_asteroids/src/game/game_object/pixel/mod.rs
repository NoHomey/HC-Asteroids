pub mod color;
pub mod position;

use self::color::*;
use self::position::*;

pub struct Pixel {
    pub color: Color,
    pub position: Position,
}

impl Pixel {
    pub fn new(color: ColorName, x: u8, y: u8) -> Pixel {
        Pixel {color: Color::new(color), position: Position::new(x, y)}
    }
}