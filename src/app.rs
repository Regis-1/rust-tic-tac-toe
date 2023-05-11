use std::io::{self, Write};

struct Marks {
    x: char,
    o: char,
}

#[derive(Debug)]
pub enum Modes {
    Ai,
    Human,
    Online,
}

pub struct App {
    marks: Marks,
}

impl App {
    pub fn default() -> Self {
        Self {
            marks: Marks { x: 'x', o: 'o' },
        }
    }

    pub fn show_menu(&self) {
        println!("Welcome to Rusty Tic-Tac-Toe!");
        self.show_mode_selection();
    }

    fn show_mode_selection(&self) {
        let modes_list = "Please select preffered game mode:
[1] - Player vs Ai
[2] - Player vs Human
[3] - Online match";
        let prompt = "-> ";
        
        print!("{modes_list}\n\n{prompt}");
        io::stdout().flush().unwrap();
    }

    pub fn mode_select(&self) -> Result<Modes, std::io::Error> {
        loop {
            let mut player_input = String::new();
            io::stdin().read_line(&mut player_input)?;

            match player_input.chars().next().unwrap() {
                '1' => return Ok(Modes::Ai),
                '2' => return Ok(Modes::Human),
                '3' => return Ok(Modes::Online),
                _ => self.show_mode_selection(),
            }
        }
    }

    pub fn run_game(&self, mode: Modes) {
        println!("Running game with the {:?} mode...", mode);
    }
}
