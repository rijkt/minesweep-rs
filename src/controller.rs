use crate::board::BoardTile;
mod controller_impl;

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
    pub (crate) width: i32, 
    pub (crate) height: i32,
}

pub(crate) trait Process {
    fn process(&mut self, requests: Vec<ControllerRequest>) -> ();
}

pub(crate) enum RequestType {
    REVEAL,
    REVEAL_AROUND,
    FLAG,
    UN_FLAG,
}

pub(crate) struct ControllerRequest {
    pub(crate) req_type: RequestType,
    pub(crate) pos: (i32, i32),
}

pub(crate) struct Controller {
    board: Vec<Vec<BoardTile>>,
    pub(crate) state: GameState, // TODO: timer
}
