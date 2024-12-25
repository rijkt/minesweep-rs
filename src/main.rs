use itertools::Itertools;
use rand::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

fn main() {
    let board = gen_board(10, 10, 10);
    assert_eq!(board.len(), 100);
    println!("{:?}", board)
}

struct Tile {
    pub pos: (i32, i32),
    pub is_mine: bool,
    pub mine_neighbors: u8,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tile")
            .field("pos", &self.pos)
            .field("is_mine", &self.is_mine)
            .field("mine_neighbors", &self.mine_neighbors)
            .finish()
    }
}

fn gen_board(width: i32, height: i32, mines: i32) -> Vec<Tile> {
    let mine_generation = gen_mines(width, height, mines);
    let mine_pos = mine_generation.0;
    let mine_count = mine_generation.1;
    (0..width)
        .cartesian_product(0..height)
        .map(|(x, y)| Tile {
            pos: (x, y),
            is_mine: mine_pos.contains(&(x, y)),
            mine_neighbors: *mine_count.get(&(x, y)).unwrap(),
        })
        .collect() // TOOD: chunk
}

fn gen_mines(max_x: i32, max_y: i32, mines: i32) -> (HashSet<(i32, i32)>, HashMap<(i32, i32), u8>) {
    let mine_pos = gen_mine_positions(max_x, max_y, mines);
    let neighbors = count_neighbors(mine_pos.clone(), max_x, max_y);
    (mine_pos, neighbors)
}

fn count_neighbors(
    mine_pos: HashSet<(i32, i32)>,
    max_x: i32,
    max_y: i32,
) -> HashMap<(i32, i32), u8> {
    (0..max_x)
        .cartesian_product(0..max_y)
        .unique()
        .map(|(x, y)| ((x, y), 0))
        .collect()
}

fn gen_mine_positions(max_x: i32, max_y: i32, mines: i32) -> HashSet<(i32, i32)> {
    let mut rng = rand::thread_rng();
    (0..mines)
        .map(|_| (rng.gen_range(0..max_x), rng.gen_range(0..max_y)))
        .collect() // deduplicate by putting into Set. TODO: generate more on duplicates.
}
