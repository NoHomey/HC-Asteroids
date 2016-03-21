extern crate hc_asteroids;

use hc_asteroids::game::game_object::GameObject;
use hc_asteroids::game::game_object::pixel::color::number_to_color;
use hc_asteroids::game::game_object::pixel::position::Position;

fn main() {
    let object: GameObject = GameObject::new(number_to_color(3), 1, 2, Position::new(0, 1));
    println!("Position: {}", object.base.position.number());
    println!("Color: {}, {}", object.base.color.value, object.base.color.to_array()[1]);
}