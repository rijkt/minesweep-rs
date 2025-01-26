use super::{PlayerSolver, Solve};
use crate::controller::{ControllerRequest, GameState, RequestType};
use std::io::{self, Write};

impl Solve for PlayerSolver {
    fn solve(&mut self, game_state: &GameState) -> Vec<ControllerRequest> {
        let input = read_input();
        if input.is_empty() {
            return vec![];
        }
        parse_request(game_state, input).map_or(vec![], |req| vec![req])
    }
}

fn read_input() -> String {
    print!(">");
    std::io::stdout()
        .flush()
        .expect("Failed to write to stdout");
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
        ["?"] => {
            print_help();
            None
        }
        [prefix, x, y] => parse_prefixed(prefix, x, y, game_state.width, game_state.height),
        _ => {
            print_invalid_msg();
            None
        }
    }
}

fn parse_prefixed(
    prefix: &str,
    token1: &str,
    token2: &str,
    max_x: i32,
    max_y: i32,
) -> Option<ControllerRequest> {
    match prefix {
        "?" => {
            print_help();
            None
        }
        "f" => parse_pos(token1, token2, max_x, max_y).map(|pos| ControllerRequest {
            pos,
            req_type: RequestType::Flag,
        }),
        "b" => todo!(),
        _ => {
            print_invalid_msg();
            None
        }
    }
}

fn print_help() {
    println!("Available commands:");
    println!("x y - reveal position (x y)");
    println!("f x y - toggle flag on position (x y)");
    println!("b x y - toggle bomb on position (x y)");
    println!("? - Print this message");
}

fn parse_pos(token1: &str, token2: &str, max_x: i32, max_y: i32) -> Option<(i32, i32)> {
    if let (Ok(x), Ok(y)) = (token1.parse::<i32>(), token2.parse::<i32>()) {
        validate_pos(x, y, max_x, max_y)
    } else {
        print_invalid_msg();
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

fn print_invalid_msg() {
    println!("Please enter a valid command. Enter \"?\" for available options");
}