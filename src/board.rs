use crate::{cell::*, player::PlayerKind};

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
            scores: [0; 2],
        }
    }
    pub fn new_board(&mut self) -> Self {
        Self {
            state: [
                [Cell::new(), Cell::new(), Cell::new()],
                [Cell::new(), Cell::new(), Cell::new()],
                [Cell::new(), Cell::new(), Cell::new()],
            ],
            scores: self.scores,
        }
    }
    pub fn increase_score(&mut self, player: &PlayerKind) {
        match player {
            PlayerKind::Noughts => self.scores[1] += 1,
            PlayerKind::Crosses => self.scores[0] += 1,
        }
    }
}

pub fn display(board: &GameBoard) {
    println!(" |A|B|C|");
    for x in 0..board.state.len() {
        print!("{}", x + 1);
        for y in &board.state[x] {
            print!("|{}", y.display);
        }
        println!("|");
    }
}

pub fn check_win(board: &GameBoard) -> bool {
    let row_1 = [&board.state[0][0], &board.state[0][1], &board.state[0][2]];
    let row_2 = [&board.state[1][0], &board.state[1][1], &board.state[1][2]];
    let row_3 = [&board.state[2][0], &board.state[2][1], &board.state[2][2]];

    let row_4 = [&board.state[0][0], &board.state[1][0], &board.state[2][0]];
    let row_5 = [&board.state[0][1], &board.state[1][1], &board.state[2][1]];
    let row_6 = [&board.state[0][2], &board.state[1][2], &board.state[2][2]];

    let row_7 = [&board.state[0][0], &board.state[1][1], &board.state[2][2]];
    let row_8 = [&board.state[2][0], &board.state[1][1], &board.state[0][2]];

    is_all_true(row_1)
        || is_all_true(row_2)
        || is_all_true(row_3)
        || is_all_true(row_4)
        || is_all_true(row_5)
        || is_all_true(row_6)
        || is_all_true(row_7)
        || is_all_true(row_8)
}

fn is_all_true(row: [&Cell; 3]) -> bool {
    row.iter().all(|&x| match &x.state {
        StateKind::Crosses => true,
        _ => false,
    }) || row.iter().all(|&x| match &x.state {
        StateKind::Noughts => true,
        _ => false,
    })
}

pub fn switch_player(current_player: PlayerKind) -> PlayerKind {
    match current_player {
        PlayerKind::Crosses => PlayerKind::Noughts,
        PlayerKind::Noughts => PlayerKind::Crosses,
    }
}
