use board::{gen_board, Tile};
use render::{ConsoleRenderer, Render};

mod board;
mod render;

fn main() {
    let mut controller = Controller::new();
    println!("{:?}", controller.board);
    let renderer = ConsoleRenderer {};
    renderer.render(&controller.state);
    let revealed = controller.reveal((3, 4));
    println!("-----");
    renderer.render(&revealed);
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
    fn reveal(&mut self, pos: (i32, i32)) -> GameState; // TODO make immutable
}

struct Controller {
    board: Vec<Vec<Tile>>,
    state: GameState, // TODO: timer
}

impl Controller {
    fn new() -> Self {
        let width = 7;
        let height = 5;
        Self {
            board: gen_board(width, height, 10), // TODO: parameterize
            state: GameState {
                board_view: vec![vec![PlayTile::hidden(); width as usize]; height as usize],
                exploded: false,
            },
        }
    }
}

impl Reveal for Controller {
    fn reveal(&mut self, pos: (i32, i32)) -> GameState {
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        let board_tile = &self.board[x][y];
        self.state.board_view[x][y] = PlayTile {
            flagged: false, // TODO
            revealed: true,
            mine_neighbors: board_tile.mine_neighbors,
            mine: board_tile.is_mine,
        };
        GameState {
            board_view: self.state.board_view.clone(),
            exploded: board_tile.is_mine,
        }
    }
}
