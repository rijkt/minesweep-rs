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

struct NumberTile {
    pub x: i32,
    pub y: i32,
    pub mine_neighbors: i32,
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

fn gen_board(width: i32, height: i32, mines: i32) {
    let mut rng = rand::thread_rng();

    let points: HashSet<(i32, i32)> = (0..mines)
        .map(|_| (rng.gen_range(0..=width), rng.gen_range(0..=height)))
        .collect();
    print!("{:?}", points)
}
