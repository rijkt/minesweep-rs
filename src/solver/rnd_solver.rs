use super::RandomSolver;
use super::Solve;
use crate::controller::ControllerRequest;
use crate::controller::GameState;
use crate::controller::RequestType;
use rand::Rng;

impl RandomSolver {
    pub(crate) fn new(rng: rand::prelude::ThreadRng) -> Self {
        Self { rng }
    }
}

impl Solve for RandomSolver {
    fn solve(&mut self, game_state: &GameState) -> Vec<ControllerRequest> {
        vec![ControllerRequest {
            req_type: RequestType::REVEAL,
            pos: get_random_pos(game_state.width, game_state.height, &mut self.rng),
        }]
    }
}

fn get_random_pos(
    width: i32,
    height: i32,
    rng: &mut rand::prelude::ThreadRng,
) -> (i32, i32) {
    (rng.gen_range(0..width), rng.gen_range(0..height))
}
