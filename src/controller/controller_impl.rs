use super::PlayTile;
use super::Process;
use super::RequestType;
use super::ControllerRequest;
use super::GameState;
use crate::board::gen_board;
use super::Controller;

impl Controller {
    pub(crate) fn new(width: i32, height: i32) -> Self {
        Self {
            board: gen_board(width, height, 10), // TODO: parameterize
            state: GameState {
                board_view: vec![vec![PlayTile::hidden(); width as usize]; height as usize],
                exploded: false,
                width,
                height
            },
        }
    }

    fn reveal(&mut self, pos: (i32, i32)) -> GameState { // TODO make immutable
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
            width: self.state.width,
            height: self.state.height,
        }
    }

    fn process_single(&mut self, request: ControllerRequest) -> () {
        match request.req_type {
            RequestType::REVEAL => {
                self.reveal(request.pos);
            }
            RequestType::REVEAL_AROUND => todo!(),
            RequestType::FLAG => todo!(),
            RequestType::UN_FLAG => todo!(),
        }
    }
}

impl Process for Controller {
    fn process(&mut self, requests: Vec<ControllerRequest>) -> () {
        requests.into_iter().for_each(|req| {
            self.process_single(req);
        });
    }
}
