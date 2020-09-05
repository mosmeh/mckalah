mod human;
mod mcts;
mod random;

pub use human::HumanPolicy;
pub use mcts::MctsPolicy;
pub use random::RandomPolicy;

use crate::game::Board;

pub trait Policy {
    fn play(&mut self, board: &Board) -> Board;
    fn on_opponents_move(&mut self, _: &Board) {}
}
