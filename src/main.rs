use crate::gesture::{CanChallenge, Gesture};
use std::fmt::Display;

mod gesture;

fn main() {
    let result = play_simulation();
    println!("{}", result);
}

// TODO: Write tests.
fn play_simulation() -> SimulationResult {
    let rock_player = || Gesture::Rock;
    let round_results = (0..100).map(|_| play_round(rock_player, rock_player));
    SimulationResult {
        round_results: round_results.collect(),
    }
}

// TODO: write tests.
fn play_round(contender: impl Fn() -> Gesture, opponent: impl Fn() -> Gesture) -> RoundResult {
    let contender_gesture = contender();
    let opponent_gesture = opponent();

    return if contender_gesture == opponent_gesture {
        RoundResult::Draw
    } else {
        if contender_gesture.wins_against(opponent_gesture) {
            RoundResult::ContenderWin
        } else {
            RoundResult::OpponentWin
        }
    };
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

#[derive(Clone)]
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
}
