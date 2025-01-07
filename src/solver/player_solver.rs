use super::{PlayerSolver, Solve};
use crate::controller::{ControllerRequest, GameState, RequestType};
use std::{io, vec};

impl Solve for PlayerSolver {
    fn solve(&mut self, game_state: &GameState) -> Vec<ControllerRequest> {
        println!("Enter position:");
        let input = read_input();
        if input.is_empty() {
            return vec![];
        }
        parse_request(game_state, input).map_or(vec![], |req| vec![req])
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_owned()
}

fn parse_request(game_state: &GameState, input: String) -> Option<ControllerRequest> {
    let input_tokens: Vec<&str> = input.split_whitespace().collect();
    match input_tokens.as_slice() {
        [x, y] => {
            parse_pos(x, y, game_state.width, game_state.height).map(|pos| ControllerRequest {
                req_type: RequestType::Reveal,
                pos,
            })
        }
        [prefix, x, y] => todo!(),
        _ => {
            println!("Please enter a valid command."); // TODO: specify
            None
        }
    }
}

fn parse_pos(token1: &str, token2: &str, max_x: i32, max_y: i32) -> Option<(i32, i32)> {
    if let (Ok(x), Ok(y)) = (token1.parse::<i32>(), token2.parse::<i32>()) {
        validate_pos(x, y, max_x, max_y)
    } else {
        None
    }
}

fn validate_pos(x: i32, y: i32, max_x: i32, max_y: i32) -> Option<(i32, i32)> {
    if x >= max_x || y >= max_y || x < 0 || y < 0 {
        println!("Stay within bounds {max_x}x{max_y}.");
        None
    } else {
        Some((x, y))
    }
}
