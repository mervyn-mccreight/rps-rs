use crate::Rng;
use enum_ordinalize::Ordinalize;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::distributions::uniform::SampleBorrow;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::uniform::UniformSampler;

#[derive(Ordinalize, PartialEq, Debug, Copy, Clone)]
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

pub struct UniformGesture(Uniform<i8>);

// TODO: Test that the uniform gives values as expected.
impl UniformSampler for UniformGesture {
    type X = Gesture;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformGesture(Uniform::new(
            low.borrow().ordinal(),
            high.borrow().ordinal(),
        ))
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformGesture(Uniform::new_inclusive(
            low.borrow().ordinal(),
            high.borrow().ordinal(),
        ))
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Gesture::from_ordinal(self.0.sample(rng))
            .expect("Underlying Uniform gave an value out of range.")
    }
}

impl SampleUniform for Gesture {
    type Sampler = UniformGesture;
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

    mod uniform {
        use super::*;

        #[test]
        #[should_panic(expected = "Uniform::new called with `low >= high`")]
        fn new_should_panic_if_lower_bound_is_greater_than_higher_bound() {
            Uniform::new(Gesture::Paper, Gesture::Rock);
        }

        #[test]
        #[should_panic(expected = "Uniform::new called with `low >= high`")]
        fn new_should_panic_if_lower_bound_is_equal_to_higher_bound() {
            Uniform::new(Gesture::Paper, Gesture::Paper);
        }

        #[test]
        #[should_panic(expected = "Uniform::new_inclusive called with `low > high`")]
        fn new_inclusive_should_panic_if_lower_bound_is_greater_than_higher_bound() {
            Uniform::new_inclusive(Gesture::Paper, Gesture::Rock);
        }
    }
}
