pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Position {
        Position {x: x, y: y}
    }
    pub fn number(&self) -> u8 {
        self.x * 8 + self.y
    }
}