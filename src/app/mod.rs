mod game;
mod opponent;
mod human_opponent;
mod ai_opponent;
mod online_opponent;

use std::io::{self, Write};
use game::Game;
use opponent::Opponent;
use human_opponent::HumanOpponent;

#[derive(Debug)]
pub enum Modes {
    Ai,
    Human,
    Online
}

pub struct App {
    should_exit: bool,
    selected_mode: Modes,
}

impl App {
    pub fn default() -> Self {
        Self {
            should_exit: false,
            selected_mode: Modes::Human,
        }
    }

    fn show_welcome_msg(&self) {
        println!("Welcome to Rusty Tic-Tac-Toe!");
    }

    fn show_available_selections(&self) {
        let modes_list = "Please select preffered game mode:
[1] - Player vs Ai
[2] - Player vs Human (local)
[3] - Online match
[9] - Quit application";
        let prompt = "-> ";
        
        print!("{modes_list}\n\n{prompt}");
        io::stdout().flush().unwrap();
    }

    fn select_mode(&mut self) -> Result<(), std::io::Error> {
        self.show_welcome_msg();

        let mut mode_selected = false;
        while !mode_selected {
            self.show_available_selections();
            mode_selected = self.read_players_choice()?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        self.select_mode()?;

        if self.should_exit {
            return Ok(());
        };

        let opponent = self.get_opponent(&self.selected_mode);
        let game = Game::new(Box::new(opponent));
        game.run();

        Ok(())
    }

    fn read_players_choice(&mut self) -> Result<bool, std::io::Error> {
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input)?;

        match player_input.chars().next().unwrap() {
            '1' => self.selected_mode = Modes::Ai,
            '2' => self.selected_mode = Modes::Human,
            '3' => self.selected_mode = Modes::Online,
            '9' => self.should_exit = true,
            _ => return Ok(false),
        }

        Ok(true)
    }

    fn get_opponent(&self, mode: &Modes) -> impl Opponent {
        match mode {
            Modes::Human => HumanOpponent::new(),
            Modes::Ai => HumanOpponent::new(),
            Modes::Online => HumanOpponent::new(),
        }
    }
}
