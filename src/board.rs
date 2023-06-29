use crate::cell::*;
pub struct GameBoard {
    pub state: [[Cell; 3]; 3],
    pub scores: [i32; 2],
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            state: [
                [Cell::new(), Cell::new(), Cell::new()],
                [Cell::new(), Cell::new(), Cell::new()],
                [Cell::new(), Cell::new(), Cell::new()],
            ],
            scores: [0, 0],
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
    false
}
