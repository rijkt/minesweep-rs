use crate::controller::{ControllerRequest, GameState};
mod player_solver;
mod rnd_solver;

pub(crate) trait Solve {
    fn solve(&mut self, game_state: &GameState) -> Vec<ControllerRequest>;
}

pub(crate) struct PlayerSolver { // TODO: hide using factory
}

pub(crate) struct RandomSolver {
    rng: rand::prelude::ThreadRng,
}