use serde::{Deserialize, Serialize};
use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn startup() {
    let point = Point { x: 1, y: 2 };
    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    // 查当前工作目录是什么
    match env::current_dir() {
        Ok(path) => println!("path = {:?}", path),
        Err(e) => println!("error = {:?}", e),
    }

    let tmpPath= Path::new("./tmp");

    if !tmpPath.exists() {
        // 如果临时目录不存在，创建临时目录
        match fs::create_dir(String::from("./tmp")) {
            Ok(_) => println!("mkdir dir"),
            Err(e) => println!("error = {:?}", e),
        }
    }

    match File::create("./tmp/point.json") {
        Ok(mut f) => {
            f.write_all(serialized.as_bytes()).unwrap();
        },
        Err(e) => println!("error = {:?}", e),
    }
}
