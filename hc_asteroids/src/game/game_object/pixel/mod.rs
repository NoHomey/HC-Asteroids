pub mod color;
pub mod position;

use self::color::*;
use self::position::*;

pub struct Pixel {
    pub col: Color,
    pub pos: Position,
}

impl Pixel {
    pub fn new(color: ColorName, x: u8, y: u8) -> Pixel {
        Pixel {col: Color::new(color), pos: Position::new(x, y)}
    }
}