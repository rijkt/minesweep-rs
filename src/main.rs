use itertools::Itertools;
use rand::prelude::*;
use std::collections::HashSet;

fn main() {
    let board = gen_board(10, 10, 10);
    assert_eq!(board.len(), 100)
}

struct Tile {
    pub pos: (i32, i32),
    pub is_mine: bool,
    pub mine_neighbors: u8,
}

fn gen_board(width: i32, height: i32, mines: i32) -> Vec<Tile> {
    let mine_coords = gen_mine_positions(width, height, mines);
    (0..width)
        .cartesian_product(0..height)
        .map(|(x, y)| Tile {
            pos: (x, y),
            is_mine: mine_coords.contains(&(x, y)),
            mine_neighbors: 0,
        })
        .collect() // TOOD: chunk
}

fn gen_mine_positions(width: i32, height: i32, mines: i32) -> HashSet<(i32, i32)> {
    let mut rng = rand::thread_rng();
    (0..mines)
        .map(|_| (rng.gen_range(0..width), rng.gen_range(0..height)))
        .collect() // deduplicate by putting into Set. TODO: generate more on duplicates.
}
