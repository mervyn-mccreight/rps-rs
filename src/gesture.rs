use std::cmp::Ordering;

use rand::{
    RngExt,
    distr::{Distribution, StandardUniform},
};
use strum::{EnumCount, FromRepr};

#[derive(EnumCount, PartialEq, Debug, Copy, Clone, FromRepr)]
pub enum Gesture {
    Rock,
    Paper,
    Scissors,
}

pub trait CanChallenge {
    fn challenge(&self, other: &Gesture) -> Ordering;
}

impl CanChallenge for Gesture {
    fn challenge(&self, other: &Gesture) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        match (self, other) {
            (Gesture::Rock, Gesture::Scissors) => Ordering::Greater,
            (Gesture::Paper, Gesture::Rock) => Ordering::Greater,
            (Gesture::Scissors, Gesture::Paper) => Ordering::Greater,
            _ => Ordering::Less,
        }
    }
}

impl Distribution<Gesture> for StandardUniform {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Gesture {
        let index = rng.random_range(0..Gesture::COUNT);
        Gesture::from_repr(index).unwrap_or(Gesture::Rock)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(Gesture::Rock, Gesture::Scissors => Ordering::Greater; "Rock wins against scissors")]
    #[test_case(Gesture::Rock, Gesture::Paper => Ordering::Less; "Rock loses against paper")]
    #[test_case(Gesture::Rock, Gesture::Rock => Ordering::Equal; "Rock draws against rock")]
    #[test_case(Gesture::Paper, Gesture::Rock => Ordering::Greater; "Paper wins against rock")]
    #[test_case(Gesture::Paper, Gesture::Scissors => Ordering::Less; "Paper loses against scissors")]
    #[test_case(Gesture::Paper, Gesture::Paper => Ordering::Equal; "Paper draws against paper")]
    #[test_case(Gesture::Scissors, Gesture::Paper => Ordering::Greater; "Scissors win against paper")]
    #[test_case(Gesture::Scissors, Gesture::Rock => Ordering::Less; "Scissors lose against rock")]
    #[test_case(Gesture::Scissors, Gesture::Scissors => Ordering::Equal; "Scissors draws against scissors")]
    fn challenge_rules_test(one: Gesture, two: Gesture) -> Ordering {
        one.challenge(&two)
    }

    #[test]
    fn distribution_test() {
        let mut rng: rand::rngs::StdRng = rand::make_rng();
        for _ in 0..100 {
            let random_gesture: Gesture = rng.random();
            assert!(
                random_gesture == Gesture::Scissors
                    || random_gesture == Gesture::Rock
                    || random_gesture == Gesture::Paper
            )
        }
    }
}
