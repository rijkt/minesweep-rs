use controller::{Controller, Reveal};
use rand::Rng;
use render::{ConsoleRenderer, Render};
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
        let pos = (rng.gen_range(0..width), rng.gen_range(0..height));
        let revealed = controller.reveal(pos);
        renderer.render(&revealed);
    }
}
