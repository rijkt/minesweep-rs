use super::CheckGameResult;
use super::Controller;
use super::ControllerRequest;
use super::GameResult;
use super::GameState;
use super::PlayTile;
use super::Process;
use super::RequestType;
use crate::board::gen_board;

impl Controller {
    pub(crate) fn new(width: i32, height: i32, mines: i32) -> Self {
        Self {
            board: gen_board(width, height, mines),
            state: GameState {
                board_view: vec![vec![PlayTile::hidden(); width as usize]; height as usize],
                exploded: false,
                width,
                height,
                mines,
            },
        }
    }

    fn reveal(&mut self, pos: (i32, i32)) -> GameState {
        // TODO make immutable
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        let board_tile = &self.board[y][x];
        let state = &mut self.state;
        state.board_view[y][x] = PlayTile {
            flagged: false, // TODO
            revealed: true,
            mine_neighbors: board_tile.mine_neighbors,
            mine: board_tile.is_mine,
        };
        state.exploded = board_tile.is_mine;
        GameState {
            board_view: state.board_view.clone(),
            exploded: board_tile.is_mine,
            width: state.width,
            height: state.height,
            mines: state.mines,
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

impl CheckGameResult for Controller {
    fn check_result(&self) -> super::GameResult {
        if self.state.exploded {
            GameResult::LOSE
        } else {
            GameResult::IN_PROGRESS
        }
    }
}
