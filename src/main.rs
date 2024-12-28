use controller::{Controller, Reveal};
use rand::Rng;
use render::{ConsoleRenderer, Render};
mod board;
mod controller;
mod render;

fn main() {
    let mut controller = Controller::new();
    let renderer = ConsoleRenderer {};
    let mut rng = rand::thread_rng();

    renderer.render(&controller.state);
    while !controller.state.exploded {
        let pos = (rng.gen_range(0..7), rng.gen_range(0..5));
        let revealed = controller.reveal(pos);
        renderer.render(&revealed);
    }
}
