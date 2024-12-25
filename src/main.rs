fn main() {
    let mine = Mine {
        x: 0,
        y: 0
    };

    assert!(matches!(mine.check(), CheckResult::BOOM))
}

enum CheckResult {
    OK,
    BOOM
}

trait Checkable {
    fn check(&self) -> CheckResult;
}

struct Mine {
    pub x: i32,
    pub y: i32
}

struct NumberTile {
    pub x: i32,
    pub y: i32,
    pub mine_neighbors: i32
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