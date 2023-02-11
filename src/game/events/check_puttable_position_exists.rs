use crate::game::{
    assets::{board::Board, square::Square}, 
    rule::{
        position::Position, 
        search::{
            search_down, 
            search_down_left, 
            search_down_right, 
            search_left, 
            search_right, 
            search_up, 
            search_up_left, 
            search_up_right
        }, 
        turn::Turn
    }
};

/// Check if any puttable position exists in the `Board` and updates the state of each `Square`.
/// 
/// # Arguments
/// 
/// * `board` - The current state of the game board.
/// * `turn` - The current player's turn.
/// 
/// # Returns
/// 
/// Returns a `Board` with updated state of each `Square`.
/// 
/// # Example
/// 
/// ```
/// use number-reversi::game::{
///     assets::{board::Board, square::Square}, 
///     rule::{
///         position::Position, 
///         search::{
///             search_down, 
///             search_down_left, 
///             search_down_right, 
///             search_left, 
///             search_right, 
///             search_up, 
///             search_up_left, 
///             search_up_right
///         }, 
///         turn::Turn,
///         color::Color
///     }
/// };
/// 
/// use number-reversi::game::events::check_puttable_position_exists::check_puttable_position_exists;
/// 
/// let mut board = Board::set_initial_state();
/// let turn = Turn{color: Color::Black};
/// 
/// let updated_board = check_puttable_position_exists(board.clone(), turn);
/// 
/// assert_ne!(board, updated_board);
/// ```
pub fn check_puttable_position_exists(board: Board, turn: Turn) -> Board {
    let mut new_board = board.clone();
    let check_functions = [
        search_down,
        search_down_left,
        search_down_right,
        search_left,
        search_right,
        search_up,
        search_up_left,
        search_up_right,
    ];
    (0..8).flat_map(|i| 
        (0..8).map(move |j| (i, j))
    ).for_each(|(i, j)| {
        match new_board.squares[i][j] {
            Square::Putted(_) => {},
            _ => {
                let position = Position{x: i, y: j};
                let candidates = 
                    check_functions.iter().flat_map(
                        |f| f(&new_board, position, turn)
                    )
                    .collect::<Vec<_>>();
                if candidates.len() != 0 {
                    new_board.squares[i][j] = Square::Puttable(candidates);
                } else {
                    new_board.squares[i][j] = Square::Empty;
                }
            }
        }
    });
    new_board
}

#[cfg(test)]
mod check_puttable_position_exists_test {
    use crate::game::{assets::{board::Board, square::Square, piece::Piece}, rule::{position::{Position, ReversibleCandidates}, turn::Turn, color::Color}};

    use super::check_puttable_position_exists;

    #[test]
    fn when_no_exist_reversible_piece_all_way_then_the_position_is_empty() {
        let board = Board::set_initial_state();
        let turn = Turn{color: Color::Black};
        let result_board = check_puttable_position_exists(board, turn);
        assert_eq!(result_board.squares[0][0], Square::Empty);
    }

    #[test]
    fn when_exist_reversible_piece_then_the_position_is_puttable() {
        let board = Board::set_initial_state();
        let turn = Turn{color: Color::Black};
        let result_board = check_puttable_position_exists(board, turn);
        assert_eq!(result_board.squares[2][4], 
            Square::Puttable(vec![
                ReversibleCandidates{ 
                    opposite: Position { x: 4, y: 4 }, 
                    positions: vec![Position { x: 3, y: 4 }], 
                }]
            )
        );
    }
    #[test]
    fn when_exist_reversible_piece_then_the_position_is_puttable2() {
        let board = Board::set_initial_state();
        let white = Color::White;
        let black = Color::Black;
        let turn = Turn{color: black};
        let board = board.put_piece(Position { x: 3, y: 5 }, Piece::new(white, 1));
        let board = board.put_piece(Position { x: 4, y: 5 }, Piece::new(white, 1));
        let board = board.put_piece(Position { x: 5, y: 4 }, Piece::new(black, 3));
        let result_board = check_puttable_position_exists(board, turn);
        assert_eq!(result_board.squares[3][6], 
            Square::Puttable(vec![
                ReversibleCandidates{ 
                    opposite: Position { x: 3, y: 3 }, 
                    positions: vec![Position { x: 3, y: 5 }, Position { x: 3, y: 4 }], 
                },
                ReversibleCandidates{ 
                    opposite: Position { x: 5, y: 4 }, 
                    positions: vec![Position { x: 4, y: 5 }], 
                }]
            )
        );
    }
}