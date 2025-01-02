use crate::controller;

use super::controller::PlayTile;
use tabled::{
    settings::{object::Rows, Border, Style},
    Table,
};

pub(crate) trait Render {
    fn render(&self, state: &controller::GameState);
}

pub(crate) struct ConsoleRenderer {}

impl Render for ConsoleRenderer {
    fn render(&self, state: &controller::GameState) {
        println!();
        let view: Vec<Vec<&str>> = state
            .board_view
            .iter()
            .map(|row| row.iter().map(render_tile).collect())
            .collect();
        let mut table = Table::from_iter(view);
        table.with(Style::ascii());
        table.modify(
            Rows::first(),
            Border::inherit(Style::ascii())
        );
        println!("{table}");
    }
}

fn render_tile(tile: &PlayTile) -> &str {
    match tile {
        PlayTile {
            pos: _,
            mine: true,
            flagged: _,
            revealed: true,
            mine_neighbors: _,
        } => "💣",
        PlayTile {
            pos: _,
            mine: false,
            flagged: _,
            revealed: true,
            mine_neighbors: num,
        } => render_number_tile(num),
        PlayTile {
            pos: _,
            mine: _,
            flagged: false,
            revealed: false,
            mine_neighbors: _,
        } => "❓",
        PlayTile {
            pos: _,
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
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        _ => panic!("Encountered invalid neighbor count {num}"),
    }
}
