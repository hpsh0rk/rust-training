use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    name: String,
    age: u8,
    hobbies: Vec<String>,
}

fn main() {
    let mut file = File::open("examples/yaml/config.yaml").expect("File not found");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("Read file failed");
    //
    // let config: Config = serde_yaml::from_str(&contents).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader).unwrap();

    println!("{:#?}", config);
}
