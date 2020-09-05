use super::Policy;
use crate::game::{Board, Player};
use rand::{rngs::SmallRng, seq::IteratorRandom, Rng, SeedableRng};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

const EXPLORATION_CONST: f32 = std::f32::consts::SQRT_2;
const EXPANSION_THRESHOLD: usize = 1;

#[derive(Debug)]
pub struct MctsPolicy {
    timeout: Duration,
    root: Rc<RefCell<Node>>,
    rng: SmallRng,
}

impl Policy for MctsPolicy {
    fn play(&mut self, board: &Board) -> Board {
        self.descend_game_tree(board);

        let time_limit = Instant::now() + self.timeout;
        while Instant::now() < time_limit {
            let mut node = self.root.clone();

            while !node.borrow().is_leaf() {
                let ln_n = (node.borrow().visits as f32).ln();
                let child = node
                    .borrow()
                    .children
                    .iter()
                    .max_by(|a, b| {
                        a.borrow()
                            .ucb(ln_n)
                            .partial_cmp(&b.borrow().ucb(ln_n))
                            .unwrap_or(std::cmp::Ordering::Less)
                    })
                    .unwrap()
                    .clone();
                node = child;
            }

            let winner = node.borrow_mut().playout(&mut self.rng);
            node.borrow_mut().visits += 1;
            if let Some(winner) = winner {
                if winner == node.borrow().parent_player {
                    node.borrow_mut().wins += 1;
                } else {
                    node.borrow_mut().loses += 1;
                }
            }

            if node.borrow().visits >= EXPANSION_THRESHOLD {
                expand_node(&node);
            }

            while let Some(parent) = node.clone().borrow().parent.upgrade() {
                parent.borrow_mut().visits += 1;
                if let Some(winner) = winner {
                    if winner == parent.borrow().parent_player {
                        parent.borrow_mut().wins += 1;
                    } else {
                        parent.borrow_mut().loses += 1;
                    }
                }
                node = parent;
            }
        }

        let next = self
            .root
            .borrow()
            .children
            .iter()
            .max_by(|a, b| {
                a.borrow()
                    .win_rate()
                    .partial_cmp(&b.borrow().win_rate())
                    .unwrap_or(std::cmp::Ordering::Less)
            })
            .unwrap()
            .clone();
        eprintln!("#visits: {}", self.root.borrow().visits);
        eprintln!("Expected win rate: {}", next.borrow().win_rate());

        self.root = next;
        self.root.borrow().board.clone()
    }

    fn on_opponents_move(&mut self, board: &Board) {
        self.descend_game_tree(board);
    }
}

impl MctsPolicy {
    pub fn new(n: u8, timeout: Duration) -> Self {
        let root = Node {
            board: Board::new(n),
            parent: Weak::new(),
            children: Vec::new(),
            visits: 0,
            wins: 0,
            loses: 0,
            parent_player: Player::Second,
        };
        Self {
            timeout,
            root: Rc::new(RefCell::new(root)),
            rng: SmallRng::from_entropy(),
        }
    }

    fn descend_game_tree(&mut self, board: &Board) {
        if self.root.borrow().children.is_empty() {
            expand_node(&self.root);
        }

        if self.root.borrow().board != *board {
            let root = self
                .root
                .borrow()
                .children
                .iter()
                .find(|x| x.borrow().board == *board)
                .unwrap()
                .clone();
            self.root = root;
        }

        if self.root.borrow().children.is_empty() {
            expand_node(&self.root);
        }

        self.root.borrow_mut().parent = Weak::new();
    }
}

#[derive(Debug)]
struct Node {
    board: Board,
    parent: Weak<RefCell<Node>>,
    children: Vec<Rc<RefCell<Node>>>,
    visits: usize,
    wins: usize,
    loses: usize,
    parent_player: Player,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn win_rate(&self) -> f32 {
        debug_assert!(self.wins + self.loses <= self.visits);

        if self.visits == 0 {
            0.5
        } else {
            0.5 + (self.wins as f32 - self.loses as f32) / (self.visits * 2) as f32
        }
    }

    fn ucb(&self, n_ln: f32) -> f32 {
        if self.visits == 0 {
            std::f32::INFINITY
        } else {
            self.win_rate() + EXPLORATION_CONST * (n_ln / self.visits as f32).sqrt()
        }
    }

    fn playout<R: Rng>(&mut self, mut rng: R) -> Option<Player> {
        let mut board = self.board.clone();
        while !board.is_game_over() {
            let next_move = board.possible_moves().choose(&mut rng).unwrap();
            board.apply_move(next_move);
        }

        board.winner()
    }
}

fn expand_node(node: &Rc<RefCell<Node>>) {
    debug_assert!(node.borrow().children.is_empty());

    let parent_player = node.borrow().board.player();
    let children = node
        .borrow()
        .board
        .next_states()
        .map(|board| {
            let child = Node {
                board,
                parent: Rc::downgrade(node),
                children: Vec::new(),
                visits: 0,
                wins: 0,
                loses: 0,
                parent_player,
            };
            Rc::new(RefCell::new(child))
        })
        .collect();
    node.borrow_mut().children = children;
}
