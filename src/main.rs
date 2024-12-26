use board::{gen_board, Tile};

mod board;

fn main() {
    let controller = Controller::new();
    assert_eq!(controller.board.len(), 100);
    println!("{:?}", controller.board)
}

struct PlayTile {
    flagged: bool,
    revealed: bool,
    mine_neighbors: Option<u8>,
    mine: bool,
}

impl PlayTile {
    fn hidden() -> Self {
        Self {
            flagged: false,
            revealed: false,
            mine: false,
            mine_neighbors: None,
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
    println!()
}
