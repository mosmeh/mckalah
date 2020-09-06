use mckalah::game::{Board, Player};
use mckalah::policy::{HumanPolicy, MctsPolicy, Policy, RandomPolicy};
use std::time::Duration;
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
#[derive(Copy, Clone)]
enum PolicyOption {
    Human,
    Random,
    Mcts,
}
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(
        number_of_values = 2,
        case_insensitive = true,
        use_delimiter = true,
        possible_values = &["human", "random", "mcts"],
        default_value = "human,mcts"
    )]
    policy: Vec<PolicyOption>,

    /// n stones in each pit
    #[structopt(short, default_value = "3")]
    n: u8,

    /// Timeout for Monte Carlo tree search in seconds
    #[structopt(short, long, default_value = "1")]
    timeout: f32,
}

fn main() {
    let opt = Opt::from_args();

    let mut board = Board::new(opt.n);

    let mut first_policy = create_policy(opt.policy[0], &opt);
    let mut second_policy = create_policy(opt.policy[1], &opt);

    while !board.is_game_over() {
        println!("{}", board);

        let next = match board.player() {
            Player::First => first_policy.play(&board),
            Player::Second => second_policy.play(&board),
        };

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

        match board.player() {
            Player::First => {
                second_policy.on_opponents_move(&next);
            }
            Player::Second => {
                first_policy.on_opponents_move(&next);
            }
        };
        board = next;

        println!()
    }

    println!(
        "{}\n\n{}",
        board,
        board
            .winner()
            .map(|p| format!("{:?} won", p))
            .unwrap_or_else(|| "Draw".to_string())
    );
}

fn create_policy(policy: PolicyOption, opt: &Opt) -> Box<dyn Policy> {
    use PolicyOption::*;

    match policy {
        Human => Box::new(HumanPolicy::default()),
        Random => Box::new(RandomPolicy::default()),
        Mcts => Box::new(MctsPolicy::new(opt.n, Duration::from_secs_f32(opt.timeout))),
    }
}
