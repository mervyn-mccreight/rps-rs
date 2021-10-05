#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Gesture {
    Rock,
    Paper,
    Scissors,
}

pub trait CanChallenge {
    fn wins_against(&self, other: Gesture) -> bool;
}

impl CanChallenge for Gesture {
    fn wins_against(&self, other: Gesture) -> bool {
        match self {
            Gesture::Paper => other == Gesture::Rock,
            Gesture::Scissors => other == Gesture::Paper,
            Gesture::Rock => other == Gesture::Scissors,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(Gesture::Rock, Gesture::Scissors => true; "Rock wins against scissors")]
    #[test_case(Gesture::Rock, Gesture::Paper => false; "Rock loses against paper")]
    #[test_case(Gesture::Paper, Gesture::Rock => true; "Paper wins against rock")]
    #[test_case(Gesture::Paper, Gesture::Scissors => false; "Paper loses against scissors")]
    #[test_case(Gesture::Scissors, Gesture::Paper => true; "Scissors win against paper")]
    #[test_case(Gesture::Scissors, Gesture::Rock => false; "Scissors lose against rock")]
    fn wins_against_rules_test(one: Gesture, two: Gesture) -> bool {
        one.wins_against(two)
    }
}
