use crate::controller;

use super::controller::PlayTile;
use itertools::Itertools;
use tabled::{
    settings::{
        object::{Cell, Columns, Rows},
        Border, Modify, Style,
    },
    Table,
};

pub(crate) trait Render {
    fn render(&self, state: &controller::GameState);
}

pub(crate) struct ConsoleRenderer {}

impl Render for ConsoleRenderer {
    fn render(&self, state: &controller::GameState) {
        println!();
        let rendered_board: Vec<Vec<String>> = state
            .board_view
            .iter()
            .map(|row| row.iter().map(render_tile).collect())
            .collect();
        let full_table_data = add_coordinate_labels(state, rendered_board);
        let table = build_table(full_table_data);
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

fn add_coordinate_labels(
    state: &controller::GameState,
    view: Vec<Vec<String>>,
) -> Vec<Vec<String>> {
    let row_labels: Vec<String> = (0..state.height).map(|x| x.to_string()).collect_vec();
    let with_row_labels = add_row_labels(&view, row_labels);
    let col_labels: Vec<String> = (0..state.width).map(|x: i32| x.to_string()).collect_vec();
    let leading_col_labels = std::iter::once("".to_owned()).chain(col_labels).collect(); // for empty corner
    std::iter::once(leading_col_labels)
        .chain(with_row_labels)
        .collect::<Vec<Vec<String>>>()
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

fn build_table(full_table: Vec<Vec<String>>) -> Table {
    Table::from_iter(full_table)
        .with(Style::extended())
        .with(Modify::new(Rows::first()).with(Border::inherit(Style::ascii().top(' '))))
        .with(Modify::new(Columns::first()).with(Border::inherit(Style::ascii().left(' '))))
        .with(Modify::new(Cell::new(0, 0)).with(Border::inherit(Style::ascii().top(' ').left(' '))))
        .to_owned()
}
