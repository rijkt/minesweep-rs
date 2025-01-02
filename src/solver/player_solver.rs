use super::{PlayerSolver, Solve};
use crate::controller::{ControllerRequest, GameState, RequestType};
use std::io;


impl Solve for PlayerSolver {
    fn solve(&mut self, game_state: &GameState) -> Vec<ControllerRequest> {
        let pos = parse_input(game_state.width, game_state.height);
        vec![ControllerRequest {
            req_type: RequestType::Reveal,
            pos,
        }]
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
