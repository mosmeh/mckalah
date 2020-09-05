use super::Policy;
use crate::game::Board;
use std::io::{self, Write};

#[derive(Debug, Default)]
pub struct HumanPolicy;

impl Policy for HumanPolicy {
    fn play(&mut self, board: &Board) -> Board {
        let mut line = String::new();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut line).unwrap();

            if let Ok(next_move) = line.trim().parse() {
                if board.is_valid_move(next_move) {
                    let mut next = board.clone();
                    next.apply_move(next_move);
                    return next;
                }
            }

            line.clear();
        }
    }
}
