extern crate hc_asteroids;

use hc_asteroids::game::game_object::pixel::Pixel;
use hc_asteroids::game::game_object::pixel::color::ColorName;

fn main() {
    let p: Pixel = Pixel::new(ColorName::Blue, 1, 2);
    println!("Position: {}", p.pos.number());
    println!("Color: {}, {}", p.col.color, p.col.to_array()[0]);
}