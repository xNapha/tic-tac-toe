use crate::board::GameBoard;
use crate::cell::{Cell, StateKind};
use crate::player::PlayerKind;

fn create_new_board() -> GameBoard {
    GameBoard::new()
}
#[test]
fn board_created_empty() {
    let test_board = create_new_board();
    let row_1 = &test_board.state()[0];
    let row_2 = &test_board.state()[1];
    let row_3 = &test_board.state()[2];

    let row_1 = row_1.iter().all(|x| match x.state() {
        StateKind::Empty => true,
        _ => false,
    });

    assert!(row_1);

    let row_2 = row_2.iter().all(|x| match x.state() {
        StateKind::Empty => true,
        _ => false,
    });

    assert!(row_2);

    let row_3 = row_3.iter().all(|x| match x.state() {
        StateKind::Empty => true,
        _ => false,
    });

    assert!(row_3);

    assert_eq!(0, test_board.scores().noughts_points());
    assert_eq!(0, test_board.scores().crosses_points());
}

#[test]
fn increasing_crosses_score() {
    let mut test_board = create_new_board();

    let crosses = PlayerKind::Crosses;

    test_board.increase_score(&crosses);

    assert_eq!(1, test_board.scores().crosses_points());
}

#[test]
fn increasing_noughts_score() {
    let mut test_board = create_new_board();

    let noughts = PlayerKind::Noughts;

    test_board.increase_score(&noughts);
    assert_eq!(1, test_board.scores().noughts_points());
}
#[test]
fn display_empty_board() {
    let test_board = create_new_board();

    let board_state = " |A|B|C|\n1| | | |\n2| | | |\n3| | | |\n";

    assert_eq!(board_state, test_board.display());
}

#[test]
fn check_win_downwards_right_diagonal() {
    let mut test_board = create_new_board();

    let mut cell = Cell::new();
    cell.update_cell(StateKind::Crosses, 'X');
    test_board.set_cell(0, 0, cell);
    test_board.set_cell(1, 1, cell);
    test_board.set_cell(2, 2, cell);

    assert!(test_board.check_win());
}

#[test]
fn check_win_downwards_left_diagonal() {
    let mut test_board = create_new_board();

    let mut cell = Cell::new();
    cell.update_cell(StateKind::Crosses, 'X');
    test_board.set_cell(0, 2, cell);
    test_board.set_cell(1, 1, cell);
    test_board.set_cell(2, 0, cell);

    assert!(test_board.check_win());
}

#[test]
fn check_win_left_column() {
    let mut test_board = create_new_board();

    let mut cell = Cell::new();
    cell.update_cell(StateKind::Crosses, 'X');
    test_board.set_cell(0, 0, cell);
    test_board.set_cell(1, 0, cell);
    test_board.set_cell(2, 0, cell);

    assert!(test_board.check_win());
}

#[test]
fn check_win_middle_column() {
    let mut test_board = create_new_board();

    let mut cell = Cell::new();
    cell.update_cell(StateKind::Crosses, 'X');
    test_board.set_cell(0, 1, cell);
    test_board.set_cell(1, 1, cell);
    test_board.set_cell(2, 1, cell);

    assert!(test_board.check_win());
}

#[test]
fn check_win_right_column() {
    let mut test_board = create_new_board();

    let mut cell = Cell::new();
    cell.update_cell(StateKind::Crosses, 'X');
    test_board.set_cell(0, 2, cell);
    test_board.set_cell(1, 2, cell);
    test_board.set_cell(2, 2, cell);

    assert!(test_board.check_win());
}

#[test]
fn check_win_top_row() {
    let mut test_board = create_new_board();

    let mut cell = Cell::new();
    cell.update_cell(StateKind::Crosses, 'X');
    test_board.set_cell(0, 0, cell);
    test_board.set_cell(0, 1, cell);
    test_board.set_cell(0, 2, cell);

    assert!(test_board.check_win());
}

#[test]
fn check_win_middle_row() {
    let mut test_board = create_new_board();

    let mut cell = Cell::new();
    cell.update_cell(StateKind::Crosses, 'X');
    test_board.set_cell(1, 0, cell);
    test_board.set_cell(1, 1, cell);
    test_board.set_cell(1, 2, cell);

    assert!(test_board.check_win());
}

#[test]
fn check_win_bottom_row() {
    let mut test_board = create_new_board();

    let mut cell = Cell::new();
    cell.update_cell(StateKind::Crosses, 'X');
    test_board.set_cell(2, 0, cell);
    test_board.set_cell(2, 1, cell);
    test_board.set_cell(2, 2, cell);

    assert!(test_board.check_win());
}

#[test]
fn switch_players() {
    let mut test_board = create_new_board();

    let crosses = match test_board.current_player() {
        PlayerKind::Crosses => true,
        PlayerKind::Noughts => false,
    };

    assert!(crosses, "Current player should be crosses");

    test_board.set_other_player();

    let noughts = match test_board.current_player() {
        PlayerKind::Crosses => false,
        PlayerKind::Noughts => true,
    };

    assert!(noughts, "Current player should now be noughts");
}
