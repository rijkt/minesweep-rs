use rand::prelude::*;
use std::{collections::HashSet, vec};

fn main() {
    gen_board(10, 10, 10)
}

struct Tile {
    pub pos: (i32, i32),
    pub is_mine: bool,
    pub mine_neighbors: u8,
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        Tile {
            pos: self.pos,
            is_mine: self.is_mine,
            mine_neighbors: self.mine_neighbors,
        }
    }
}

fn gen_board(width: i32, height: i32, mines: i32) {
    let mine_coords = gen_mine_positions(width, height, mines);
    let mut board: Vec<Vec<Tile>> = vec![Vec::with_capacity(width as usize); height as usize];
    mine_coords.iter().for_each(|pos| {
        board[pos.0 as usize][pos.1 as usize] = Tile {
            pos: *pos,
            is_mine: mine_coords.contains(pos),
            mine_neighbors: 0, // TODO
        }
    });
}

fn gen_mine_positions(width: i32, height: i32, mines: i32) -> HashSet<(i32, i32)> {
    let mut rng = rand::thread_rng();
    (0..mines)
        .map(|_| (rng.gen_range(0..width), rng.gen_range(0..height)))
        .collect() // deduplicate by putting into Set. TODO: generate more on duplicates.
}
