#[derive(Debug)]
pub enum StateKind {
    Noughts,
    Crosses,
    Empty,
}
#[derive(Debug)]
pub struct Cell {
    pub state: StateKind,
    pub display: char,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            state: StateKind::Empty,
            display: ' ',
        }
    }
}
