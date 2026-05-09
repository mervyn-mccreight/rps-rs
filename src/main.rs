#![warn(clippy::pedantic, clippy::panic, clippy::nursery)]
#![deny(
    clippy::all,
    clippy::unwrap_used,
    clippy::todo,
    clippy::expect_used,
    clippy::enum_glob_use
)]

use crate::gesture::{CanChallenge, Gesture};
use crate::player::Player;
use rand::RngExt;
use rand::rng;
use std::cmp::Ordering;
use std::fmt::Display;

mod gesture;
mod player;

fn main() {
    let result = play_simulation();
    println!("{result}");
}

fn play_simulation() -> SimulationResult {
    let mut rng = rng();
    let mut rock_player = || Gesture::Rock;
    let mut random_player = move || rng.random::<Gesture>();
    let round_results = (0..100).map(|_| play_round(&mut rock_player, &mut random_player));
    SimulationResult {
        round_results: round_results.collect(),
    }
}

fn play_round(contender: &mut dyn Player, opponent: &mut dyn Player) -> RoundResult {
    let contender_gesture = contender.decide();
    let opponent_gesture = opponent.decide();

    match contender_gesture.challenge(opponent_gesture) {
        Ordering::Equal => RoundResult::Draw,
        Ordering::Greater => RoundResult::ContenderWin,
        Ordering::Less => RoundResult::OpponentWin,
    }
}

#[derive(Clone)]
struct SimulationResult {
    round_results: Vec<RoundResult>,
}

impl Display for SimulationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wins: u8 = self
            .round_results
            .clone()
            .into_iter()
            .filter_map(|r| match r {
                RoundResult::ContenderWin => Some(1),
                _ => None,
            })
            .sum();
        let draws: u8 = self
            .round_results
            .clone()
            .into_iter()
            .filter_map(|r| match r {
                RoundResult::Draw => Some(1),
                _ => None,
            })
            .sum();
        let losses: u8 = self
            .round_results
            .clone()
            .into_iter()
            .filter_map(|r| match r {
                RoundResult::OpponentWin => Some(1),
                _ => None,
            })
            .sum();
        write!(f, "Wins={wins}, Draws={draws}, Losses={losses}")
    }
}

#[derive(Clone, Debug, PartialEq)]
enum RoundResult {
    ContenderWin,
    OpponentWin,
    Draw,
}

#[cfg(test)]
mod tests {
    use super::*;
    mod simulation_result {
        use super::*;

        #[test]
        fn should_print_an_empty_result() {
            let display = format!(
                "{}",
                SimulationResult {
                    round_results: vec![]
                }
            );
            assert_eq! {display, "Wins=0, Draws=0, Losses=0"}
        }

        #[test]
        fn should_print_a_result_with_multiple_wins() {
            let display = format!(
                "{}",
                SimulationResult {
                    round_results: vec![RoundResult::ContenderWin, RoundResult::ContenderWin]
                }
            );
            assert_eq! {display, "Wins=2, Draws=0, Losses=0"}
        }

        #[test]
        fn should_print_a_result_with_multiple_draws() {
            let display = format!(
                "{}",
                SimulationResult {
                    round_results: vec![RoundResult::Draw, RoundResult::Draw]
                }
            );
            assert_eq! {display, "Wins=0, Draws=2, Losses=0"}
        }

        #[test]
        fn should_print_a_result_with_multiple_losses() {
            let display = format!(
                "{}",
                SimulationResult {
                    round_results: vec![RoundResult::OpponentWin, RoundResult::OpponentWin]
                }
            );
            assert_eq! {display, "Wins=0, Draws=0, Losses=2"}
        }

        #[test]
        fn should_print_a_result_with_mixed_results() {
            let display = format!(
                "{}",
                SimulationResult {
                    round_results: vec![
                        RoundResult::ContenderWin,
                        RoundResult::Draw,
                        RoundResult::OpponentWin
                    ]
                }
            );
            assert_eq! {display, "Wins=1, Draws=1, Losses=1"}
        }
    }

    mod play_round {
        use super::*;

        #[test]
        fn should_return_draw() {
            let mut scissors_player_one = || Gesture::Scissors;
            let mut scissors_player_two = || Gesture::Scissors;
            assert_eq!(
                play_round(&mut scissors_player_one, &mut scissors_player_two),
                RoundResult::Draw
            );
        }

        #[test]
        fn should_return_win() {
            let mut scissors_player = || Gesture::Scissors;
            let mut paper_player = || Gesture::Paper;

            assert_eq!(
                play_round(&mut scissors_player, &mut paper_player),
                RoundResult::ContenderWin
            );
        }

        #[test]
        fn should_return_loss() {
            let mut scissors_player = || Gesture::Scissors;
            let mut rock_player = || Gesture::Rock;

            assert_eq!(
                play_round(&mut scissors_player, &mut rock_player),
                RoundResult::OpponentWin
            );
        }

        #[test]
        fn should_work_multiple_times() {
            let mut scissors_player_one = || Gesture::Scissors;
            let mut scissors_player_two = || Gesture::Scissors;
            assert_eq!(
                play_round(&mut scissors_player_one, &mut scissors_player_two),
                RoundResult::Draw
            );
            assert_eq!(
                play_round(&mut scissors_player_one, &mut scissors_player_two),
                RoundResult::Draw
            );
        }
    }
}
