use rand::prelude::*;
use std::{collections::HashSet, vec};

fn main() {
    let mine = Mine { x: 0, y: 0 };

    assert!(matches!(mine.check(), CheckResult::BOOM));
    gen_board(10, 10, 10)
}

enum CheckResult {
    OK,
    BOOM,
}

trait Checkable {
    fn check(&self) -> CheckResult;
}

struct Mine {
    pub x: i32,
    pub y: i32,
}

impl Clone for Mine {
    fn clone(&self) -> Self {
        Mine {
            x: self.x,
            y: self.y
        }
    }
}

struct NumberTile {
    pub x: i32,
    pub y: i32,
    pub mine_neighbors: u8,
}

impl Checkable for Mine {
    fn check(&self) -> CheckResult {
        CheckResult::BOOM
    }
}

impl Checkable for NumberTile {
    fn check(&self) -> CheckResult {
        CheckResult::OK
    }
}

struct InitializingTile {}

impl Checkable for InitializingTile {
    fn check(&self) -> CheckResult {
        panic!("Encountered initializing tile")
    }
}

impl Clone for InitializingTile {
    fn clone(&self) -> Self {
        InitializingTile {}
    }
}

fn gen_board(width: i32, height: i32, mines: i32) {
    let mut rng = rand::thread_rng();
    let mine_coords: HashSet<(i32, i32)> = (0..mines)
        .map(|_| (rng.gen_range(0..=width), rng.gen_range(0..=height)))
        .collect();
    let init = Mine {x: -1, y: -1};
    let mut board: Vec<Vec<Mine>> = vec![vec![init; width as usize]; height as usize];
    mine_coords
        .iter()
        .for_each(|(x, y)| board[*x as usize][*y as usize] = Mine{
            x: *x,
            y: *y
        });
}
