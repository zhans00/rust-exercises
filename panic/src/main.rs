use std::fs::File;
use std::fs;
use panic::*;

fn main() {
    let filename = "created.txt";
    File::create(filename).unwrap();
    let a = open_file(filename);
    println!("{:?}", a);
    fs::remove_file(filename).unwrap();
}
