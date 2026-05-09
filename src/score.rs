use crate::RoundResult;

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Score {
    pub wins: usize,
    pub draws: usize,
    pub losses: usize,
}

impl std::iter::Sum for Score {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |a, b| Self {
            wins: a.wins + b.wins,
            draws: a.draws + b.draws,
            losses: a.losses + b.losses,
        })
    }
}

impl From<&RoundResult> for Score {
    fn from(value: &RoundResult) -> Self {
        match value {
            RoundResult::ContenderWin => Self {
                wins: 1,
                draws: 0,
                losses: 0,
            },
            RoundResult::OpponentWin => Self {
                wins: 0,
                draws: 0,
                losses: 1,
            },
            RoundResult::Draw => Self {
                wins: 0,
                draws: 1,
                losses: 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(vec![] => Score::default(); "The sum of an empty set of scores is the default score")]
    #[test_case(vec![Score { wins: 1, draws: 1, losses: 1 }] => Score { wins: 1, draws: 1, losses: 1}; "The sum of one score is the score itself")]
    #[test_case(vec![Score { wins: 1, draws: 1, losses: 1 }, Score::default()] => Score { wins: 1, draws: 1, losses: 1}; "The default score is the neutral element")]
    #[test_case(vec![Score { wins: 1, draws: 1, losses: 1 }, Score { wins: 2, draws: 3, losses: 4 }] => Score { wins: 3, draws: 4, losses: 5}; "The sum of scores is defined as the sum of wins, draws and losses")]
    fn score_sum_test(scores: Vec<Score>) -> Score {
        scores.into_iter().sum()
    }

    #[test_case(&RoundResult::ContenderWin => Score { wins: 1, draws: 0, losses: 0})]
    #[test_case(&RoundResult::OpponentWin => Score { wins: 0, draws: 0, losses: 1})]
    #[test_case(&RoundResult::Draw => Score { wins: 0, draws: 1, losses: 0})]
    fn from_round_result_definition(round_result: &RoundResult) -> Score {
        Score::from(round_result)
    }
}
