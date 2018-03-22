use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn parse_file(file_name: &str) {
    let file = File::open(file_name).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);
    println!("{}",contents);
    assert_eq!(contents, "Hello handsome!\nHello beautiful.\nI am happy to see you.\nI like you.\nYou smell nice.\nYou smell good.\nYou smell very sweet.\nYou are beautiful.");
}