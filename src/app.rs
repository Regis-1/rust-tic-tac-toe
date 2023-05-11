struct Marks {
    x: char,
    o: char,
}

enum Modes {
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

    pub fn mode_select(&self) {
        let welcome_msg = "Welcome to Rusty Tic-Tac-Toe!
Please select preffered game mode:
[1] - Player vs Ai
[2] - Player vs Human
[3] - Online match";
        let prompt = "-> ";

        print!("{welcome_msg}\n\n{prompt}");
    }
}
