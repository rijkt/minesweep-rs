use board::{gen_board, Tile};
use itertools::Itertools;

mod board;

fn main() {
    let controller = Controller::new();
    assert_eq!(controller.board.len(), 100);
    println!("{:?}", controller.board);
    render(controller.state);
}

struct PlayTile {
    flagged: bool,
    revealed: bool,
    mine_neighbors: u8,
    mine: bool,
}

impl PlayTile {
    fn hidden() -> Self {
        Self {
            flagged: false,
            revealed: false,
            mine: false,
            mine_neighbors: 0,
        }
    }
}

impl Clone for PlayTile {
    fn clone(&self) -> Self {
        Self {
            flagged: self.flagged.clone(),
            revealed: self.revealed.clone(),
            mine_neighbors: self.mine_neighbors.clone(),
            mine: self.mine.clone(),
        }
    }
}

struct GameState {
    board_view: Vec<Vec<PlayTile>>,
    exploded: bool, // maybe moves/history
}

trait Reveal {
    fn reveal(&self, pos: (i32, i32)) -> GameState;
}

struct Controller {
    board: Vec<Tile>,
    state: GameState, // TODO: timer
                      // TODO: renderer
}

impl Controller {
    fn new() -> Self {
        Self {
            board: gen_board(10, 10, 10), // TODO: parameterize
            state: GameState {
                board_view: vec![vec![PlayTile::hidden(); 10]; 10],
                exploded: false,
            },
        }
    }
}

impl Reveal for Controller {
    fn reveal(&self, pos: (i32, i32)) -> GameState {
        GameState {
            board_view: todo!(),
            exploded: todo!(),
        }
    }
}

fn render(state: GameState) -> () {
    let view = state
        .board_view
        .iter()
        .map(|row| {
            row.iter()
                .map(|tile| render_tile(tile))
                .join("|")
        })
        .join("\n");
    println!("{}", view);
}

fn render_tile(tile: &PlayTile) -> &str {
    match tile {
        PlayTile {
            mine: true,
            flagged: _,
            revealed: true,
            mine_neighbors: _,
        } => "💣",
        PlayTile {
            mine: false,
            flagged: _,
            revealed: true,
            mine_neighbors: num,
        } => match num {
            1 => "1️⃣",
            2 => "2️⃣",
            3 => "3️⃣",
            4 => "4️⃣",
            5 => "5️⃣",
            6 => "6️⃣",
            7 => "7️⃣",
            8 => "8️⃣",
            _ => panic!("Encountered invalid neighbor count {}", num),
        },
        PlayTile {
            mine: _,
            flagged: false,
            revealed: false,
            mine_neighbors: _,
        } => "❓",
        PlayTile {
            mine: _,
            flagged: true,
            revealed: false,
            mine_neighbors: _,
        } => "🚩",
    }
}
