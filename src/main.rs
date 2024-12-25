use itertools::Itertools;
use rand::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    f64,
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
        .map(|pos| {
            (
                pos,
                mine_pos
                    .iter()
                    .map(|mine_pos| distance(pos, *mine_pos))
                    .filter(|d| *d == 1)
                    .count() as u8,
            )
        })
        .collect()
}

fn distance(v1: (i32, i32), v2: (i32, i32)) -> u8 {
    let d1 = (v2.0 - v1.0) as f64;
    let d2 = (v2.1 - v1.1) as f64;
    (d1.powi(2) + d2.powi(2)).sqrt() as u8 // we only care about integers
}

fn gen_mine_positions(max_x: i32, max_y: i32, mines: i32) -> HashSet<(i32, i32)> {
    let mut rng = rand::thread_rng();
    (0..mines)
        .map(|_| (rng.gen_range(0..max_x), rng.gen_range(0..max_y)))
        .collect() // deduplicate by putting into Set. TODO: generate more on duplicates.
}
