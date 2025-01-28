use crate::controller;

use super::controller::PlayTile;
use itertools::Itertools;
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
        let col_labels = (0..state.width).map(|x| x.to_string()).collect_vec();
        let row_labels = (0..state.height).map(|x| x.to_string()).collect_vec();
        let mut augmented_data: Vec<Vec<&str>> = Vec::new();
        for (i, row) in view.iter().enumerate() {
            let mut new_row = vec![row_labels[i]];
            new_row.extend(row.iter().copied());
            augmented_data.push(new_row);
        }
    
        // Flatten the data into a single vector of rows
        let full_table = std::iter::once(header) // TODO: add leading space
        .chain(augmented_data.iter()).collect::<Vec<_>>();
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
