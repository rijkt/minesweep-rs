use crate::board::BoardTile;
mod controller_impl;
mod play_tile_impl;

pub(crate) struct PlayTile {
    pub(crate) pos: (i32, i32),
    pub(crate) flagged: bool,
    pub(crate) revealed: bool,
    pub(crate) mine_neighbors: u8,
    pub(crate) mine: bool,
}

pub(crate) struct GameState {
    pub(crate) board_view: Vec<Vec<PlayTile>>,
    pub(crate) exploded: bool, // maybe moves/history
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) mines: i32,
}

pub(crate) trait Process {
    fn process(&mut self, requests: Vec<ControllerRequest>) -> ();
}

pub(crate) enum GameResult {
    WIN,
    LOSE,
    IN_PROGRESS
}

pub(crate) trait CheckGameResult {
    fn check_result(&self) -> GameResult;
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
