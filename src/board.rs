use crate::{cell::*, player::PlayerKind};

#[derive(Copy, Clone)]
pub struct GameBoard {
    state: [[Cell; 3]; 3],
    scores: Scores,
    current_player: PlayerKind,
}

#[derive(Copy, Clone)]
pub struct Scores {
    noughts_points: i32,
    crosses_points: i32,
}

impl Scores {
    pub fn new() -> Self {
        Self {
            noughts_points: 0,
            crosses_points: 0,
        }
    }
    pub fn noughts_points(&self) -> i32 {
        self.noughts_points
    }

    pub fn crosses_points(&self) -> i32 {
        self.crosses_points
    }
}

impl GameBoard {
    pub fn state(&self) -> [[Cell; 3]; 3] {
        self.state
    }

    pub fn current_player(&self) -> PlayerKind {
        self.current_player
    }

    pub fn set_cell(&mut self, pos_x: usize, pos_y: usize, new_cell: Cell) {
        self.state[pos_x][pos_y] = new_cell;
    }

    pub fn scores(&self) -> Scores {
        self.scores
    }

    pub fn new() -> Self {
        Self {
            state: [
                [Cell::new(), Cell::new(), Cell::new()],
                [Cell::new(), Cell::new(), Cell::new()],
                [Cell::new(), Cell::new(), Cell::new()],
            ],
            scores: Scores::new(),
            current_player: PlayerKind::Crosses,
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
            current_player: match self.current_player {
                PlayerKind::Crosses => PlayerKind::Noughts,
                PlayerKind::Noughts => PlayerKind::Crosses,
            },
        }
    }
    pub fn increase_score(&mut self, player: &PlayerKind) {
        match player {
            PlayerKind::Noughts => self.scores.noughts_points += 1,
            PlayerKind::Crosses => self.scores.crosses_points += 1,
        }
    }

    pub fn display(&self) -> String {
        let mut board_text = String::from(" |A|B|C|\n");
        for x in 0..self.state().len() {
            board_text += (x + 1).to_string().as_str();
            for y in self.state()[x] {
                board_text += format!("|{}", y.display()).to_string().as_str();
            }
            board_text += "|\n";
        }
        board_text
    }

    pub fn check_win(&self) -> bool {
        let row_1 = [&self.state[0][0], &self.state[0][1], &self.state[0][2]];
        let row_2 = [&self.state[1][0], &self.state[1][1], &self.state[1][2]];
        let row_3 = [&self.state[2][0], &self.state[2][1], &self.state[2][2]];

        let row_4 = [&self.state[0][0], &self.state[1][0], &self.state[2][0]];
        let row_5 = [&self.state[0][1], &self.state[1][1], &self.state[2][1]];
        let row_6 = [&self.state[0][2], &self.state[1][2], &self.state[2][2]];

        let row_7 = [&self.state[0][0], &self.state[1][1], &self.state[2][2]];
        let row_8 = [&self.state[2][0], &self.state[1][1], &self.state[0][2]];

        is_all_true(row_1)
            || is_all_true(row_2)
            || is_all_true(row_3)
            || is_all_true(row_4)
            || is_all_true(row_5)
            || is_all_true(row_6)
            || is_all_true(row_7)
            || is_all_true(row_8)
    }

    pub fn set_other_player(&mut self) {
        match self.current_player {
            PlayerKind::Crosses => self.current_player = PlayerKind::Noughts,
            PlayerKind::Noughts => self.current_player = PlayerKind::Crosses,
        }
    }
}

fn is_all_true(row: [&Cell; 3]) -> bool {
    row.iter().all(|&x| match &x.state() {
        StateKind::Crosses => true,
        _ => false,
    }) || row.iter().all(|&x| match &x.state() {
        StateKind::Noughts => true,
        _ => false,
    })
}
