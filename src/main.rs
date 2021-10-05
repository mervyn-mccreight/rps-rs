use crate::gesture::{CanChallenge, Gesture};

mod gesture;

fn main() {
    let one = Gesture::Paper;
    let two = Gesture::Scissors;
    println! {"{:?} plays against {:?} and win={:?}", one, two, one.wins_against(two)};
}
