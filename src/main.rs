use mckalah::game::{Board, Player};
use mckalah::policy::{HumanPolicy, MctsPolicy, Policy, RandomPolicy};
use std::str::FromStr;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Copy, Clone)]
enum PolicyOption {
    Human,
    Random,
    Mcts,
}

impl FromStr for PolicyOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PolicyOption::*;

        match &s.to_ascii_lowercase()[..] {
            "human" => Ok(Human),
            "random" => Ok(Random),
            "mcts" => Ok(Mcts),
            _ => Err(format!("Unknown policy {}", s)),
        }
    }
}

#[derive(StructOpt)]
struct Opt {
    /// One of human, random, or mcts
    #[structopt(default_value = "human")]
    first: PolicyOption,

    #[structopt(default_value = "mcts")]
    second: PolicyOption,

    /// n stones in each pit
    #[structopt(short, default_value = "3")]
    n: u8,

    /// Timeout for Monte Carlo tree search in ms
    #[structopt(short, long, default_value = "1000")]
    timeout: u64,
}

fn main() {
    let opt = Opt::from_args();

    let mut board = Board::new(opt.n);

    let mut first_policy = create_policy(opt.first, &opt);
    let mut second_policy = create_policy(opt.second, &opt);

    while !board.is_game_over() {
        println!("{}", board);

        let next = match board.player() {
            Player::First => {
                let next = first_policy.play(&board);
                second_policy.on_opponents_move(&next);
                next
            }
            Player::Second => {
                let next = second_policy.play(&board);
                first_policy.on_opponents_move(&next);
                next
            }
        };

        assert!(board.next_states().any(|x| x == next));

        println!(
            "-> {}",
            board
                .possible_moves()
                .find(|x| {
                    let mut board = board.clone();
                    board.apply_move(*x);
                    board == next
                })
                .unwrap()
        );

        board = next;

        println!()
    }

    println!("{}", board);
    println!("{:?} won", board.winner().unwrap());
}

fn create_policy(policy: PolicyOption, opt: &Opt) -> Box<dyn Policy> {
    use PolicyOption::*;

    match policy {
        Human => Box::new(HumanPolicy::default()),
        Random => Box::new(RandomPolicy::default()),
        Mcts => Box::new(MctsPolicy::new(opt.n, Duration::from_millis(opt.timeout))),
    }
}
