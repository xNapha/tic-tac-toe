pub enum StateKind {
    Noughts,
    Crosses,
    Empty,
}
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
