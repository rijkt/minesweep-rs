use controller::{Controller, Process};
use render::{ConsoleRenderer, Render};
use solver::{PlayerSolver, Solve};
mod board;
mod controller;
mod render;

fn main() {
    let width = 7;
    let height = 5;
    let mut controller = Controller::new(width, height);
    let renderer = ConsoleRenderer {};
    let mut solver = PlayerSolver{};

    renderer.render(&controller.state);
    while !controller.state.exploded {
        let req = solver.solve(&controller.state);
        controller.process(req);
        renderer.render(&controller.state);
    }
}

mod solver {
    use crate::controller::{ControllerRequest, GameState, RequestType};
    use rand::Rng;
    use std::io;

    pub(crate) trait Solve {
        fn solve(&mut self, game_state: &GameState) -> Vec<ControllerRequest>;
    }

    pub(crate) struct PlayerSolver {}

    impl Solve for PlayerSolver {
        fn solve(&mut self, game_state: &GameState) -> Vec<ControllerRequest> {
            let pos = parse_input(game_state.width, game_state.height);
            vec![ControllerRequest {
                req_type: RequestType::REVEAL,
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

    pub(crate) struct RandomSolver {
        rng: rand::prelude::ThreadRng,
    }
    
    impl RandomSolver {
        pub(crate) fn new(rng: rand::prelude::ThreadRng) -> Self {
        Self { rng }
    }
    }

    impl Solve for RandomSolver {
        fn solve(&mut self, game_state: &GameState) -> Vec<ControllerRequest> {
            vec![ControllerRequest {
                req_type: RequestType::REVEAL,
                pos: get_random_pos(game_state.width, game_state.height, &mut self.rng),
            }]
        }
    }

    fn get_random_pos(width: i32, height: i32, rng: &mut rand::prelude::ThreadRng) -> (i32, i32) {
        (rng.gen_range(0..width), rng.gen_range(0..height))
    }
}
