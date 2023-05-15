use super::opponent::Opponent;

pub struct AiOpponent {
    name: &'static str,
}

impl Opponent for AiOpponent {
    fn name(&self) -> &str {
        self.name
    }
}
