use super::opponent::Opponent;

pub struct HumanOpponent {
    name: &'static str
}

impl Opponent for HumanOpponent {
    fn name(&self) -> &str {
        self.name
    }
}

impl HumanOpponent {
    pub fn new() -> Self {
        Self {
            name: "Player 2",
        }
    }
}
