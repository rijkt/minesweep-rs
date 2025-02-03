use std::collections::HashSet;

use super::BoardTile;
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
        let board = gen_board(width, height, mines);
        let board_view: Vec<Vec<PlayTile>> = board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|board_tile| PlayTile::hidden(board_tile.pos))
                    .collect()
            })
            .collect();
        Self {
            board,
            state: GameState {
                board_view,
                exploded: false,
                width,
                height,
                mines,
            },
        }
    }

    fn process_single(&mut self, request: ControllerRequest) {
        match request.req_type {
            RequestType::Reveal => self.reveal(request.pos),
            RequestType::Flag => self.flag(request.pos),
        }
    }

    fn reveal(&mut self, pos: (i32, i32)) {
        // TODO make immutable
        let (x, y) = (pos.0 as usize, pos.1 as usize);
        let state = &mut self.state;
        let current = &state.board_view[y][x];
        if !current.flagged {
            if current.revealed {
                get_safe_neighbors(pos, state.width, state.height)
                    .iter()
                    .for_each(|neigbor| reveal_single(*neigbor, state, &self.board));
            } else {
                reveal_single(pos, state, &self.board);
            }
        }
    }

    fn flag(&mut self, pos: (i32, i32)) {
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        let prev = &self.state.board_view[y][x];
        if !prev.revealed {
            self.state.board_view[y][x] = PlayTile {
                pos,
                flagged: !prev.flagged,
                revealed: false,
                mine_neighbors: prev.mine_neighbors,
                mine: false,
            }
        }
    }
}

fn get_safe_neighbors(pos: (i32, i32), width: i32, height: i32) -> Vec<(i32, i32)> {
    let (x, y) = (pos.0, pos.1);
    [
        // row above
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        // current row
        (x - 1, y),
        (x + 1, y),
        // row below
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .iter()
    .cloned()
    .filter(|neighbor| {
        neighbor.0 >= 0 && neighbor.0 <= width && neighbor.1 >= 0 && neighbor.1 <= height
    })
    .collect()
}

fn reveal_single(pos: (i32, i32), state: &mut GameState, board: &[Vec<BoardTile>]) {
    let (x, y) = (pos.0 as usize, pos.1 as usize);
    let board_tile = &board[y][x];
    let mine = board_tile.is_mine;
    state.board_view[y][x] = PlayTile {
        pos,
        flagged: false,
        revealed: true,
        mine_neighbors: board_tile.mine_neighbors,
        mine,
    };
    state.exploded = state.exploded || mine
}

impl Process for Controller {
    fn process(&mut self, requests: Vec<ControllerRequest>) {
        for req in requests {
            self.process_single(req);
        }
    }
}

impl CheckGameResult for Controller {
    fn check_result(&self) -> super::GameResult {
        if self.state.exploded {
            GameResult::Lose
        } else if found_all_mines(&self.board, &self.state.board_view) {
            GameResult::Win
        } else {
            GameResult::InProgress
        }
    }
}

// if needed: optimize by keeping a runnig tally
fn found_all_mines(board: &[Vec<BoardTile>], solver_view: &[Vec<PlayTile>]) -> bool {
    let mine_positions: HashSet<(i32, i32)> = board
        .iter()
        .flat_map(|row| row.iter())
        .filter(|board_tile| board_tile.is_mine)
        .map(|mine_tile| mine_tile.pos)
        .collect();
    let hidden_positions: HashSet<(i32, i32)> = solver_view
        .iter()
        .flat_map(|row| row.iter())
        .filter(|play_tile| !play_tile.revealed)
        .map(|hidden_tile| hidden_tile.pos)
        .collect();
    hidden_positions == mine_positions
}
