use itertools::Itertools;
use rand::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::iter::from_fn;

pub(crate) struct BoardTile {
    pub pos: (i32, i32),
    pub is_mine: bool,
    pub mine_neighbors: u8,
}

impl Debug for BoardTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tile")
            .field("pos", &self.pos)
            .field("is_mine", &self.is_mine)
            .field("mine_neighbors", &self.mine_neighbors)
            .finish()
    }
}

pub(crate) fn gen_board(width: i32, height: i32, mines: i32) -> Vec<Vec<BoardTile>> {
    let mine_generation = gen_mines(width, height, mines);
    let mine_pos = mine_generation.mine_pos;
    let mine_count = mine_generation.neighbors;
    (0..width)
        .cartesian_product(0..height)
        .sorted_by(|x, y| x.1.cmp(&y.1)) // column-wise
        .map(|(x, y)| BoardTile {
            pos: (x, y),
            is_mine: mine_pos.contains(&(x, y)),
            mine_neighbors: *mine_count.get(&(x, y)).unwrap(),
        })
        .chunks(width as usize)
        .into_iter()
        .map(std::iter::Iterator::collect)
        .collect()
}

struct GenMinesResult{
    mine_pos: HashSet<(i32, i32)>, 
    neighbors: HashMap<(i32, i32), u8>
}

fn gen_mines(max_x: i32, max_y: i32, mines: i32) -> GenMinesResult {
    let mine_pos: HashSet<(i32, i32)> = gen_rnd_positions(max_x, max_y)
        .unique()
        .take(mines as usize)
        .collect();
    assert_eq!(mine_pos.len(), mines as usize);
    let neighbors = count_neighbors(mine_pos.clone(), max_x, max_y);
    GenMinesResult{mine_pos, neighbors}
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
    let d1 = f64::from(v2.0 - v1.0);
    let d2 = f64::from(v2.1 - v1.1);
    (d1.powi(2) + d2.powi(2)).sqrt() as u8 // we only care about integers
}

fn gen_rnd_positions(max_x: i32, max_y: i32) -> impl Iterator<Item = (i32, i32)> {
    let mut rng = rand::thread_rng();
    from_fn(move || Some((rng.gen_range(0..max_x), rng.gen_range(0..max_y))))
}
