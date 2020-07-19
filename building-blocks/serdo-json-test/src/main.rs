use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;

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

    println!("{:?}", a);
    let j = serde_json::to_string(&a).unwrap();
    println!("{}", j);
    let mut buffer = File::create("json.txt").unwrap();
    buffer.write(j.as_bytes()).unwrap();

    let mut f = File::open("json.txt").unwrap();
    let mut b_json = String::new();
    f.read_to_string(&mut b_json).unwrap();

    let b: Move = serde_json::from_str(&b_json).unwrap();
    println!("{:?}", b);
}