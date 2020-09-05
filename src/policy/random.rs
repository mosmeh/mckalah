use super::Policy;
use crate::game::Board;
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

#[derive(Debug)]
pub struct RandomPolicy {
    rng: SmallRng,
}

impl Default for RandomPolicy {
    fn default() -> Self {
        Self {
            rng: SmallRng::from_entropy(),
        }
    }
}

impl Policy for RandomPolicy {
    fn play(&mut self, board: &Board) -> Board {
        let next_move = board.possible_moves().choose(&mut self.rng).unwrap();
        let mut next = board.clone();
        next.apply_move(next_move);
        next
    }
}
