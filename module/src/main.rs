// src/main.rs
use module::{area_of,volume_of};

const WIDTH: i32 = 4;
const HEIGHT: i32 = 7;
const DEPTH: i32 = 10;

fn main() {

    println!("Area is {} \nVolume is {}", area_of(WIDTH, HEIGHT), volume_of(WIDTH, HEIGHT, DEPTH));

}

