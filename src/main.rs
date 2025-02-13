use controller::{CheckGameResult, Controller, GameResult, Process};
use render::{ConsoleRenderer, Render};
use solver::{PlayerSolver, Solve};
mod board;
mod controller;
mod render;
mod solver;

fn main() {
    let width = 7;
    let height = 5;
    let mines = 10;
    let mut controller = Controller::new(width, height, mines);
    let renderer = ConsoleRenderer {};
    let mut solver = PlayerSolver {};
    renderer.render(&controller.state);
    while matches!(controller.check_result(), GameResult::InProgress) {
        let requests = solver.solve(&controller.state);
        controller.process(requests);
        renderer.render(&controller.state);
    }
    match controller.check_result() {
        GameResult::Win => println!("You win!"),
        GameResult::Lose => println!("You lose.."),
        _ => panic!("Illegal state"),
    }
}
