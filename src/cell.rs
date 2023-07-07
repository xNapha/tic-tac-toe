#[derive(Copy, Clone)]
pub enum StateKind {
    Noughts,
    Crosses,
    Empty,
}

#[derive(Copy, Clone)]
pub struct Cell {
    state: StateKind,
    display: char,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            state: StateKind::Empty,
            display: ' ',
        }
    }

    pub fn state(&self) -> StateKind {
        self.state
    }

    pub fn display(&self) -> char {
        self.display
    }

    pub fn update_cell(&mut self, new_state: StateKind, new_display: char) {
        self.state = new_state;
        self.display = new_display;
    }
}
