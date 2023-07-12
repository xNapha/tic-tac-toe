use crate::cell::*;

#[test]
fn newly_created_cell_empty() {
    let cell = Cell::new();

    let empty_cell = match cell.state() {
        StateKind::Empty => true,
        _ => false,
    };

    assert!(
        empty_cell,
        "cell should be created to have a state of empty "
    );

    let display = cell.display();

    assert_eq!(
        ' ', display,
        "dispaly should return a singluar space character"
    );
}

#[test]
fn updating_a_cell_to_crosses() {
    let mut cell = Cell::new();
    cell.update_cell(StateKind::Crosses, 'X');

    let crosses_cell = match cell.state() {
        StateKind::Crosses => true,
        _ => false,
    };

    assert!(
        crosses_cell,
        "cell should be created to have a state of crosses"
    );

    let display = cell.display();

    assert_eq!('X', display, "dispaly should return an X character");
}

#[test]
fn updating_a_cell_to_noughts() {
    let mut cell = Cell::new();
    cell.update_cell(StateKind::Noughts, 'O');

    let crosses_cell = match cell.state() {
        StateKind::Noughts => true,
        _ => false,
    };

    assert!(
        crosses_cell,
        "cell should be updated to have a state of noughts"
    );

    let display = cell.display();

    assert_eq!('O', display, "dispaly should return an O character");
}
