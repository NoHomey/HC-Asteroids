pub mod pixel;

use self::pixel::Pixel;
use self::pixel::position::Position;
use self::pixel::color::*;

pub struct GameObject {
    pub base: Pixel,
    pub figure: Vec<Position>,
    pub alive: u8,
    pub change: Position, 
}

impl GameObject {
    pub fn new(color: ColorName, x: u8, y: u8, change: Position) -> GameObject {
        GameObject {
            base: Pixel::new(color, x, y),
            figure: vec![],
            alive: 0,
            change: change,
        }
    }
}