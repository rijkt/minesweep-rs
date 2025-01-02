use crate::board::BoardTile;

pub mod play_tile;
mod controller_impl;

pub(crate) struct GameState {
    pub(crate) board_view: Vec<Vec<play_tile::PlayTile>>,
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

