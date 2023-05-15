use super::opponent::Opponent;

struct Marks {
    x: char,
    o: char,
}

pub struct Game {
    marks: Marks,
    opponent: Box<dyn Opponent>,
}

impl Game {
   pub fn new(opponent: Box<dyn Opponent>) -> Self {
       Self { 
           marks: Marks {x: 'x', o: 'o'},
           opponent,
       }
   }

   pub fn run(&self) {
       // 1. X always start first, so draw who will be Xs and who will be Os
       // 2. interchangeable turns:
       //   2.1 player's turn
       //   2.2 opponent's turn (getting their move)
       // 3. checking the board for win
       // 4. getting the result and the winner
   }
}
