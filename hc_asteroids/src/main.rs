extern crate hc_asteroids;

use hc_asteroids::game::game_object::color::*;
use hc_asteroids::game::game_object::position::*;

fn main() {
    let pos: Position = Position::new(0, 0);
    let col: Color = Color::new(ColorName::Blue);
    println!("Position: {}, {}", pos.x, pos.y);
    println!("Color: {}, {}, {}", col.r, col.b, col.g);
}