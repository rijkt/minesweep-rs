use controller::{Controller, Process};
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
    while !controller.state.exploded {
        let req = solver.solve(&controller.state);
        controller.process(req);
        renderer.render(&controller.state);
    }
}
