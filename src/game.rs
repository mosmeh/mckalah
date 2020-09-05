use std::fmt;
use std::str::FromStr;

// Kalah(m, n) notation: m pits per side
const KALAH_M: usize = 6;

const NUM_HOLES: usize = (KALAH_M + 1) * 2;
const PLAYER_STORE: usize = KALAH_M;
const OPPONENT_STORE: usize = NUM_HOLES - 1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    First,
    Second,
}

impl Player {
    pub fn other(self) -> Player {
        match self {
            Player::First => Player::Second,
            Player::Second => Player::First,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Move(pub usize);

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0 + 1)
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        if let Ok(i) = s.parse::<usize>() {
            if 0 < i && i <= KALAH_M {
                return Ok(Move(i - 1));
            }
        }
        Err(())
    }
}

// opponent's house: holes[12]    holes[11] ... holes[7]
// opponent's store: holes[13]  player's store: holes[6]
// player's house:   holes[0]     holes[1]  ... holes[5]

// Initial state:
// 3  3  3  3  3  3
// 0              0
// 3  3  3  3  3  3

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    holes: [u8; NUM_HOLES],
    player: Player,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "Player: {:?}", self.player)?;

        for i in (PLAYER_STORE + 1..OPPONENT_STORE).rev() {
            write!(f, "{:>2} ", self.holes[i])?;
        }

        write!(f, "\n{:>2} ", self.holes[OPPONENT_STORE])?;
        for _ in 0..KALAH_M - 2 {
            write!(f, "   ")?;
        }
        writeln!(f, "{:>2}", self.holes[PLAYER_STORE])?;

        for i in 0..PLAYER_STORE {
            write!(f, "{:>2} ", self.holes[i])?;
        }

        Ok(())
    }
}

impl Board {
    pub fn new(n: u8) -> Self {
        let mut holes = [n; NUM_HOLES];
        holes[PLAYER_STORE] = 0;
        holes[OPPONENT_STORE] = 0;

        Self {
            holes,
            player: Player::First,
        }
    }

    pub fn player(&self) -> Player {
        self.player
    }

    pub fn is_valid_move(&self, next_move: Move) -> bool {
        next_move.0 < KALAH_M && self.holes[next_move.0] > 0
    }

    pub fn apply_move(&mut self, next_move: Move) {
        debug_assert!(self.is_valid_move(next_move));

        let mut remaining_stones = self.holes[next_move.0];
        self.holes[next_move.0] = 0;

        let mut i = next_move.0 + 1;
        let mut additional_move = false;

        loop {
            remaining_stones -= 1;
            self.holes[i] += 1;

            if remaining_stones == 0 {
                if i < KALAH_M {
                    let opposite = OPPONENT_STORE - i - 1;
                    if self.holes[i] == 1 && self.holes[opposite] > 0 {
                        self.holes[PLAYER_STORE] += 1 + self.holes[opposite];
                        self.holes[i] = 0;
                        self.holes[opposite] = 0;
                    }
                } else if i == PLAYER_STORE {
                    additional_move = true;
                }

                break;
            }

            i = (i + 1) % OPPONENT_STORE;
        }

        let house_sum = self.house_sum();
        if house_sum.0 == 0 {
            self.holes[OPPONENT_STORE] += house_sum.1;
            for i in PLAYER_STORE + 1..OPPONENT_STORE {
                self.holes[i] = 0;
            }
        } else if house_sum.1 == 0 {
            self.holes[PLAYER_STORE] += house_sum.0;
            for i in 0..PLAYER_STORE {
                self.holes[i] = 0;
            }
        } else if !additional_move {
            self.holes.rotate_left(KALAH_M + 1);
            self.player = self.player.other();
        }

        debug_assert_eq!(self.holes.iter().sum::<u8>() % 2, 0);
    }

    pub fn is_game_over(&self) -> bool {
        let house_sum = self.house_sum();
        let game_over = house_sum.0 == 0 || house_sum.1 == 0;

        if game_over {
            debug_assert_eq!(house_sum, (0, 0));
        }

        game_over
    }

    pub fn winner(&self) -> Option<Player> {
        if !self.is_game_over() {
            return None;
        }

        if self.holes[PLAYER_STORE] >= self.holes[OPPONENT_STORE] {
            Some(self.player)
        } else {
            Some(self.player.other())
        }
    }

    pub fn possible_moves(&self) -> impl Iterator<Item = Move> + '_ {
        (0..KALAH_M)
            .map(Move)
            .filter(move |m| self.is_valid_move(*m))
    }

    pub fn next_states(&self) -> impl Iterator<Item = Board> + '_ {
        self.possible_moves().map(move |m| {
            let mut next = self.clone();
            next.apply_move(m);
            next
        })
    }

    fn house_sum(&self) -> (u8, u8) {
        (
            self.holes[..PLAYER_STORE].iter().sum(),
            self.holes[PLAYER_STORE + 1..OPPONENT_STORE].iter().sum(),
        )
    }
}
