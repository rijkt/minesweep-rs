use controller::{Controller, Reveal};
use render::{ConsoleRenderer, Render};
mod board;
mod controller;
mod render;

fn main() {
    let mut controller = Controller::new();
    let renderer = ConsoleRenderer {};
    renderer.render(&controller.state);
    let revealed = controller.reveal((3, 4));
    println!("-----");
    renderer.render(&revealed);
}
