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
    let mut rng = rand::thread_rng();
    let mine_coords: HashSet<(i32, i32)> = (0..mines)
        .map(|_| (rng.gen_range(0..width), rng.gen_range(0..height)))
        .collect();
    let mut board: Vec<Vec<Tile>> = vec![Vec::with_capacity(width as usize); height as usize];
    mine_coords.iter().for_each(|(x, y)| {
        board[*x as usize][*y as usize] = Tile {
            pos: (*x, *y),
            is_mine: true,
            mine_neighbors: 0, // TODO
        }
    });
}
