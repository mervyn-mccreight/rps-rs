use crate::gesture::Gesture;

pub trait Player {
    fn decide(&mut self) -> Gesture;
}

impl<F> Player for F
where
    F: FnMut() -> Gesture,
{
    fn decide(&mut self) -> Gesture {
        self()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn define_player_as_function() {
        let mut player = || Gesture::Paper;
        assert_eq!(player.decide(), Gesture::Paper);
    }
}
