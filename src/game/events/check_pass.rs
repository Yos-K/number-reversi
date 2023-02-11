use crate::game::{
    rule::turn::Turn, 
    assets::{board::Board, piece::UsedPiece, pieces::Pieces}
};

use super::check_puttable_position_exists::check_puttable_position_exists;

/// The `check_pass` function is used to determine if a move can be made or 
/// if it's time to pass the turn to the next player.
/// 
/// If a puttable position exists, the function will return the current turn, 
/// the `UsedPiece` list, the `Pieces` list, and the current `Board` state as is.
/// 
/// If a puttable position does not exist, the function will change the turn to the next player, 
/// remove a piece from the `UsedPiece` list, add it to the `Pieces` list, and update the current `Board` state.
/// 
/// # Arguments
/// 
/// * `turn` - The current player's turn.
/// * `board` - The current game board.
/// * `used` - A list of pieces that have already been placed on the board.
/// * `pieces` - A list of pieces that have not yet been placed on the board.
/// 
/// # Returns
/// 
/// A tuple containing the updated turn, `UsedPiece` list, `Pieces` list, and `Board` state.
/// 
/// # Example
/// 
/// ```
/// use number-reversi::game::assets::pieces::Pieces;
/// use number-reversi::game::assets::piece::Piece;
/// use number-reversi::game::assets::piece::UsedPiece;
/// use number-reversi::game::assets::board::Board;
/// use number-reversi::game::rule::{turn::Turn, color::Color};
/// 
/// use number-reversi::game::events::check_pass::check_pass;
/// 
/// let turn = Turn{color: Color::Black};
/// let board = Board::set_initial_state();
/// let used = UsedPiece::new().add_piece(
///     Piece::new(Color::White, 1)).add_piece(Piece::new(Color::Black, 1));
/// let pieces = Pieces::make_pieces();
/// 
/// let (new_turn, new_used, new_pieces, new_board) = check_pass(turn, board, used, pieces);
/// ```
pub fn check_pass(turn: Turn, board: Board, used: UsedPiece, pieces: Pieces) 
-> (Turn, UsedPiece, Pieces, Board) {
    if board.has_puttable() {
        (turn, used, pieces, board)
    } else {
        let turn = turn.change();
        let add_piece = used.get_add_piece(turn);
        let used = used.remove_piece(add_piece);
        let pieces = pieces.add(&add_piece);
        let board = check_puttable_position_exists(board, turn);
        (turn, used, pieces, board)
    }
}


#[cfg(test)]
mod check_pass_test {
    use std::collections::HashSet;

    use crate::{
        tests::helper::board_helper::BoardHelper, 
        game::{
            rule::{turn::Turn, color::Color, position::{ReversibleCandidates, Position}}, 
            assets::{pieces::Pieces, piece::{UsedPiece, Piece}, square::Square}
        }
    };

    use super::check_pass;

    #[test]
    fn when_no_puttable_square_then_pass() {
        let turn =Turn{color: Color::Black};
        let board = BoardHelper::make_board(
            vec![
                (1, 1, 1)], 
            vec![
                (2, 0, 1)
            ]);
        let pieces = Pieces::make_pieces();
        let used = UsedPiece::new();
        let used = used.add_piece(Piece::new(Color::White, 1));

        let (new_turn, used, pieces, board) = check_pass(turn, board, used, pieces);
        assert_eq!(new_turn, Turn{color: Color::White});
        assert_eq!(used.piece_set, HashSet::new());
        assert_eq!(pieces.get_rest_num(1, new_turn), 6);
        assert_eq!(board.squares[0][2], Square::Puttable(vec![
            ReversibleCandidates::new(
                Position{x: 2, y: 0},
                vec![Position{x: 1, y: 1}])
            ]));
    }
}