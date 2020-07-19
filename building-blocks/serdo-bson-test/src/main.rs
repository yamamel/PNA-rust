use bson;
use serde::{Serialize, Deserialize};
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
    steps: i32, // bson only support signed type
    direction: Direction,
}

fn main() {
    // let a = Move {
    //     steps: 5,
    //     direction: Direction::UP,
    // };

    // let a_bson = bson::to_bson(&a).unwrap();

    // println!("{}", a_bson);

    // let b: Move = bson::from_bson(a_bson).unwrap();
    // println!("{:?}", b);

    let mut f = File::create("bson.txt").expect("fail to create the file");

    for i in 0..10 {
        let tmp = Move {
            steps: i,
            direction: Direction::UP,
        };

        let tmp_bson = bson::to_bson(&tmp).unwrap(); // structs convert to bson are reservered in Bson.Document
    
        // f.write(&tmp_bson).unwrap();
        // println!("{:?}", tmp_bson);
        // tmp_bson 的类型是 `Bson`，Bson 中的 Document 有一个 `to_writer()` 方法
        // 但 tmp_bson.as_document() 直接就得到内部的 OrderedDocument 类了
        // 而 OrderedDocument 类不存在 to_writer 的方法，应该怎么做
        // tmp_bson.to_writer(&mut f).unwrap();

    }
}