use super::opponent::Opponent;

pub struct OnlineOpponent {
    name: &'static str,
}

impl Opponent for OnlineOpponent {
    fn name(&self) -> &str {
        self.name
    }
}
