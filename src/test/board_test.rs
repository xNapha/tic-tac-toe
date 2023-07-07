use crate::board::GameBoard;
use crate::cell::StateKind;
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
