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
        let view: Vec<Vec<String>> = state
            .board_view
            .iter()
            .map(|row| row.iter().map(render_tile).collect())
            .collect();
        let col_labels: Vec<String> = (0..state.width).map(|x: i32| x.to_string()).collect_vec();
        let row_labels: Vec<String> = (0..state.height).map(|x| x.to_string()).collect_vec();
        let augmented_data = add_row_labels(&view, row_labels);
    
        // Flatten the data into a single vector of rows
        let full_table: Vec<Vec<String>> = std::iter::once(col_labels) // TODO: add leading space at 0, 0
        .chain(augmented_data).collect::<Vec<Vec<String>>>();
        let mut table = Table::from_iter(full_table);
        table.with(Style::ascii());
        table.modify(
            Rows::first(),
            Border::inherit(Style::ascii())
        );
        println!("{table}");
    }
}

fn render_tile(tile: &PlayTile) -> String {
    let rendered = match tile {
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
    };
    rendered.to_owned()
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

fn add_row_labels(view: &[Vec<String>], row_labels: Vec<String>) -> Vec<Vec<String>> {
    let mut augmented_data: Vec<Vec<String>> = Vec::new();
    for (i, row) in view.iter().enumerate() {
        let mut new_row = vec![row_labels[i].clone()]; // could be moved though
        new_row.extend(row.clone());
        augmented_data.push(new_row);
    }
    augmented_data
}
