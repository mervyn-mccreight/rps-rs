extern crate enum_ordinalize;
extern crate rand;

use crate::gesture::{CanChallenge, Gesture};
use crate::rand::distributions::Uniform;
use crate::rand::{thread_rng, Rng};
use std::fmt::Display;
use std::iter::repeat_with;

mod gesture;

fn main() {
    let result = play_simulation();
    println!("{}", result);
}

fn play_simulation() -> SimulationResult {
    let rock_player = repeat_with(|| Gesture::Rock);
    let rng = thread_rng();

    let choices = Uniform::new_inclusive(Gesture::Rock, Gesture::Scissors);
    let mut random_player = rng.sample_iter(choices);

    let round_results = (0..100).map(|_| play_round(rock_player, &mut random_player));
    SimulationResult {
        round_results: round_results.collect(),
    }
}

fn play_round(
    contender: impl IntoIterator<Item = Gesture>,
    opponent: impl IntoIterator<Item = Gesture>,
) -> RoundResult {
    let contender_gesture = contender
        .into_iter()
        .next()
        .expect("Player unexpectedly stopped to choose a gesture.");
    let opponent_gesture = opponent
        .into_iter()
        .next()
        .expect("Player unexpectedly stopped to choose a gesture.");

    if contender_gesture == opponent_gesture {
        RoundResult::Draw
    } else if contender_gesture.wins_against(opponent_gesture) {
        RoundResult::ContenderWin
    } else {
        RoundResult::OpponentWin
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
        write! {f, "Wins={}, Draws={}, Losses={}", wins, draws, losses}
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
            let display = format! {"{}", SimulationResult { round_results: vec![] }};
            assert_eq! {display, "Wins=0, Draws=0, Losses=0"}
        }

        #[test]
        fn should_print_a_result_with_multiple_wins() {
            let display = format! {"{}", SimulationResult { round_results: vec![RoundResult::ContenderWin, RoundResult::ContenderWin] }};
            assert_eq! {display, "Wins=2, Draws=0, Losses=0"}
        }

        #[test]
        fn should_print_a_result_with_multiple_draws() {
            let display = format! {"{}", SimulationResult { round_results: vec![RoundResult::Draw, RoundResult::Draw] }};
            assert_eq! {display, "Wins=0, Draws=2, Losses=0"}
        }

        #[test]
        fn should_print_a_result_with_multiple_losses() {
            let display = format! {"{}", SimulationResult { round_results: vec![RoundResult::OpponentWin, RoundResult::OpponentWin] }};
            assert_eq! {display, "Wins=0, Draws=0, Losses=2"}
        }

        #[test]
        fn should_print_a_result_with_mixed_results() {
            let display = format! {"{}", SimulationResult { round_results: vec![RoundResult::ContenderWin, RoundResult::Draw, RoundResult::OpponentWin] }};
            assert_eq! {display, "Wins=1, Draws=1, Losses=1"}
        }
    }

    mod play_round {
        use super::*;

        #[test]
        fn should_return_draw() {
            let scissors_player = repeat_with(|| Gesture::Scissors);
            assert_eq!(
                play_round(scissors_player, scissors_player),
                RoundResult::Draw
            );
        }

        #[test]
        fn should_return_win() {
            let scissors_player = repeat_with(|| Gesture::Scissors);
            let paper_player = repeat_with(|| Gesture::Paper);

            assert_eq!(
                play_round(scissors_player, paper_player),
                RoundResult::ContenderWin
            );
        }

        #[test]
        fn should_return_loss() {
            let scissors_player = repeat_with(|| Gesture::Scissors);
            let rock_player = repeat_with(|| Gesture::Rock);

            assert_eq!(
                play_round(scissors_player, rock_player),
                RoundResult::OpponentWin
            );
        }

        #[test]
        fn should_work_multiple_times() {
            let scissors_player = repeat_with(|| Gesture::Scissors);
            assert_eq!(
                play_round(scissors_player, scissors_player),
                RoundResult::Draw
            );
            assert_eq!(
                play_round(scissors_player, scissors_player),
                RoundResult::Draw
            );
        }
    }
}
