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
        let pos = parse_input(width, height);
        let revealed = controller.reveal(pos);
        renderer.render(&revealed);
    }
}

fn parse_input(max_x: i32, max_y: i32) -> (i32, i32) {
    println!("Enter position to reveal:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return parse_input(max_x, max_y);
    }
    let numbers: Vec<i32> = trimmed
        .split_whitespace()
        .map(|num| num.parse::<i32>().expect("Invalid number"))
        .collect();
    let x = numbers[0];
    let y = numbers[1];
    if numbers.len() != 2 {
        println!("Please enter exactly two numbers.");
        parse_input(max_x, max_y)
    } else if x >= max_x || y >= max_y || x < 0 || y < 0 {
        println!("Stay within bounds {}x{}.", max_x, max_y);
        parse_input(max_x, max_y)
    } else {
        (numbers[0], numbers[1])
    }
}

fn get_random_pos(width: i32, height: i32, rng: &mut rand::prelude::ThreadRng) -> (i32, i32) {
    (rng.gen_range(0..width), rng.gen_range(0..height))
}
