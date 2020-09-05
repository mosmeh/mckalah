use super::Policy;
use crate::game::Board;
use std::io::BufRead;

#[derive(Debug, Default)]
pub struct HumanPolicy;

impl Policy for HumanPolicy {
    fn play(&mut self, board: &Board) -> Board {
        for line in std::io::stdin().lock().lines() {
            if let Ok(next_move) = line.unwrap().trim().parse() {
                if board.is_valid_move(next_move) {
                    let mut next = board.clone();
                    next.apply_move(next_move);
                    return next;
                }
            }
        }

        unreachable!()
    }
}
