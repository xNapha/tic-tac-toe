use crate::cell::*;
use crate::player::*;
pub struct GameBoard {
    pub state: [[Cell; 3]; 3],
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            state: [
                [Cell::new(), Cell::new(), Cell::new()],
                [Cell::new(), Cell::new(), Cell::new()],
                [Cell::new(), Cell::new(), Cell::new()],
            ],
        }
    }
}

pub fn display(board: &mut GameBoard) {
    println!(" |A|B|C|");
    for x in 0..board.state.len() {
        print!("{}", x + 1);
        for y in &board.state[x] {
            print!("|{}", y.display);
        }
        println!("|");
    }
}

pub fn checkWin(board: &GameBoard) -> bool {
    true
}
