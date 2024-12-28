use controller::{Controller, Reveal};
use rand::Rng;
use render::{ConsoleRenderer, Render};
use std::io;
mod board;
mod controller;
mod render;

fn main() {
    let width = 7;
    let height = 5;
    let mut controller = Controller::new(width, height);
    let renderer = ConsoleRenderer {};
    let mut rng = rand::thread_rng();

    renderer.render(&controller.state);
    while !controller.state.exploded {
        let pos = parse_input();
        let revealed = controller.reveal(pos);
        renderer.render(&revealed);
    }
}

fn parse_input() -> (i32, i32) {
    println!("Enter position to reveal:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<i32>().expect("Invalid number"))
        .collect();
    if numbers.len() != 2 {
        println!("Please enter exactly two numbers.");
        parse_input()
    } else {
        (numbers[0], numbers[1])
    }
}
fn get_random_pos(width: i32, height: i32, rng: &mut rand::prelude::ThreadRng) -> (i32, i32) {
    (rng.gen_range(0..width), rng.gen_range(0..height))
}
