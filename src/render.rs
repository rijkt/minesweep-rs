use crate::controller;

use super::controller::PlayTile;
use itertools::Itertools;

pub(crate) trait Render {
    fn render(&self, state: &controller::GameState) -> ();
}

pub(crate) struct ConsoleRenderer {}

impl Render for ConsoleRenderer {
    fn render(&self, state: &controller::GameState) -> () {
        let view = state
            .board_view
            .iter()
            .map(|row| row.iter().map(|tile| render_tile(tile)).join("|"))
            .join("\n");
        println!("{}", view);
    }
}

fn render_tile(tile: &PlayTile) -> &str {
    match tile {
        PlayTile {
            mine: true,
            flagged: _,
            revealed: true,
            mine_neighbors: _,
        } => "💣",
        PlayTile {
            mine: false,
            flagged: _,
            revealed: true,
            mine_neighbors: num,
        } => render_number_tile(num),
        PlayTile {
            mine: _,
            flagged: false,
            revealed: false,
            mine_neighbors: _,
        } => "❓",
        PlayTile {
            mine: _,
            flagged: true,
            revealed: false,
            mine_neighbors: _,
        } => "🚩",
    }
}

fn render_number_tile(num: &u8) -> &str {
    match num {
        0 => "⬜",
        1 => "1️⃣",
        2 => "2️⃣",
        3 => "3️⃣",
        4 => "4️⃣",
        5 => "5️⃣",
        6 => "6️⃣",
        7 => "7️⃣",
        8 => "8️⃣",
        _ => panic!("Encountered invalid neighbor count {}", num),
    }
}
