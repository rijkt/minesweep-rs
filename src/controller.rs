use crate::board::{gen_board, BoardTile};

pub(crate) struct PlayTile {
    pub(crate) flagged: bool,
    pub(crate) revealed: bool,
    pub(crate) mine_neighbors: u8,
    pub(crate) mine: bool,
}

impl PlayTile {
    pub(crate) fn hidden() -> Self {
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

pub(crate) struct GameState {
    pub(crate) board_view: Vec<Vec<PlayTile>>,
    pub(crate) exploded: bool, // maybe moves/history
}

pub(crate) trait Reveal {
    fn reveal(&mut self, pos: (i32, i32)) -> GameState; // TODO make immutable
}

pub(crate) struct Controller {
    board: Vec<Vec<BoardTile>>,
    pub(crate) state: GameState, // TODO: timer
}

impl Controller {
    pub(crate) fn new() -> Self {
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
        let board_tile = &self.board[y][x];
        self.state.board_view[y][x] = PlayTile {
            flagged: false, // TODO
            revealed: true,
            mine_neighbors: board_tile.mine_neighbors,
            mine: board_tile.is_mine,
        };
        self.state.exploded = board_tile.is_mine;
        GameState {
            board_view: self.state.board_view.clone(),
            exploded: board_tile.is_mine,
        }
    }
}
