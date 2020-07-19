use ron::ser::to_string;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Debug, Serialize, Deserialize)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    steps: u32,
    direction: Direction,
}

fn main() {
    let a = Move {
        steps: 5,
        direction: Direction::UP,
    };

    let _s = to_string(&a).expect("Serialization failed");
    let buffer: Vec<u8> = _s.into_bytes();

    let b = str::from_utf8(&buffer).unwrap();
    println!("{}", b);

}