use super::{PlayerSolver, Solve};
use crate::controller::{ControllerRequest, GameState, RequestType};
use std::io;

impl Solve for PlayerSolver {
    fn solve(&mut self, game_state: &GameState) -> Vec<ControllerRequest> {
        println!("Enter position:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return vec![];
        }
        let pos_opt = parse_pos(trimmed, game_state.width, game_state.height);
        pos_opt.map_or(vec![], |pos| {
            vec![ControllerRequest {
                req_type: RequestType::Reveal,
                pos,
            }]
        })
    }
}

fn parse_pos(input: &str, max_x: i32, max_y: i32) -> Option<(i32, i32)> {
    if let Ok(numbers) = parse_nums(input) {
        if numbers.len() != 2 {
            println!("Please enter exactly two numbers.");
            None
        } else if numbers[0] >= max_x || numbers[1] >= max_y || numbers[0] < 0 || numbers[1] < 0 {
            println!("Stay within bounds {max_x}x{max_y}.");
            None
        } else {
            Some((numbers[0], numbers[1]))
        }
    } else {
        None
    }
}

fn parse_nums(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input
        .split_whitespace()
        .map(|num| num.parse::<i32>())
        .collect()
}
