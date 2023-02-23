use crate::{game::{rule::position::Position, assets::{piece::{Piece, UsedPiece}, board::Board, pieces::Pieces}}, app::game::GameModeProps};
use crate::game::rule::turn::Turn;
use crate::game::assets::square::Square;

use super::{check_puttable_position_exists::check_puttable_position_exists, reverse::reverse, check_pass::check_pass};

/// The `put_piece` function updates the state of the game board, pieces, and used pieces
/// when a piece is placed at a given position.
/// 
/// # Arguments
/// * `position` - The position on the board where the piece will be placed
/// * `piece` - The piece to be placed
/// * `board` - The current state of the game board
/// * `turn` - The current turn
/// * `pieces` - The remaining pieces of each color
/// * `used` - The pieces that have already been placed
/// 
/// # Returns
/// A tuple containing the updated board, turn, pieces, piece, and used pieces.
/// 
/// # Example
/// ```
/// use number-reversi::game::assets::{board::Board, pieces::Pieces, piece::{Piece, UsedPiece}};
/// use number-reversi::game::rule::{position::Position, turn::Turn, color::Color};
/// use number-reversi::game::events::put_piece::put_piece;
/// 
/// use number-reversi::game::events::check_puttable_position_exists::check_puttable_position_exists;
/// 
/// let turn = Turn{color: Color::Black};
/// let board = check_puttable_position_exists(Board::set_initial_state(), turn);
/// let pieces = Pieces::make_pieces();
/// let piece = Piece::new(Color::Black, 1);
/// let used = UsedPiece::new();
/// 
/// let position = Position { x: 4, y: 2 };
/// let (updated_board, updated_turn, updated_pieces, updated_piece, updated_used) = put_piece(position, piece, board.clone(), turn, pieces.clone(), used.clone());
/// 
/// assert_ne!(board, updated_board);
/// assert_ne!(turn, updated_turn);
/// assert_ne!(pieces, updated_pieces);
/// assert_ne!(piece, updated_piece);
/// assert_ne!(used, updated_used);
/// ```
pub fn put_piece(
    position: Position, 
    piece: Piece, 
    board: Board, 
    turn: Turn, 
    pieces: Pieces, 
    used: UsedPiece,
    game_mode_props: GameModeProps,
) -> (Board, Turn, Pieces, Piece, UsedPiece) {
    match &board.squares[position.x][position.y] {
        Square::Puttable(c) => {
            let board = board.put_piece(position, piece);
            let board = reverse(board, piece, c.to_vec());
            let board = check_puttable_position_exists(board, turn);
            
            let pieces = pieces.remove(&piece);
            let used = used.add_piece(piece);

            let turn = turn.change(game_mode_props);
            let board = check_puttable_position_exists(board, turn);

            let (turn, used, pieces, board) = check_pass(turn, board, used, pieces, game_mode_props);
            (board, turn, pieces, Piece::new(turn.color.clone(), 1), used)
        },
        _ => (
            board, turn, pieces, piece, used
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::{game::{
        assets::{board::Board, pieces::Pieces, piece::{Piece, UsedPiece}, player::Player},
        rule::{position::{Position, ReversibleCandidates}, color::Color},
        rule::turn::Turn, events::check_puttable_position_exists::check_puttable_position_exists,
    }, tests::helper::board_helper::BoardHelper, app::game::GameModeProps};
    use crate::game::assets::square::Square;
    use crate::game::events::put_piece::put_piece;

    #[test]
    fn test_put_piece() {
        let pieces = Pieces::make_pieces();
        let used = UsedPiece::new();

        let board = Board::set_initial_state();
        let props = GameModeProps{black: Player::Human, white: Player::Human};
        let turn = Turn{color: Color::Black, player: Player::Human};
        let piece = Piece::new(turn.color.clone(), 1);
        let position = Position{ x: 4, y: 2 };


        let board = check_puttable_position_exists(board, turn);

        let (new_board, new_turn, new_pieces, new_piece, new_used) = 
            put_piece(position, piece, board, turn, pieces, used, props);

        assert_eq!(
            new_board.squares[position.x][position.y], 
            Square::Put(piece)
        );
        assert_eq!(new_turn.color, Color::White);
        assert_eq!(new_pieces.black.get(&1), Some(&4));
    }

    #[test]
    fn when_next_white_piece_is_not_puttable_then_white_turn_pass() {
        let board = BoardHelper::make_board(
            vec![(0, 0, 1)],
            vec![(1, 0, 1), (0, 1, 1)]
        );
        let pieces = Pieces::make_pieces();
        let used = UsedPiece::new();
        let turn = Turn{color: Color::Black, player: Player::Human};

        let board = check_puttable_position_exists(board, turn);
        let piece = Piece::new(turn.color.clone(), 1);
        let position = Position { x: 2, y: 0 };

        let props = GameModeProps{black: Player::Human, white: Player::Human};

        let (new_board, new_turn, new_pieces, new_piece, new_used) = 
            put_piece(position, piece, board, turn, pieces, used, props);

        assert_eq!(new_turn, turn);
        assert_eq!(new_board.squares[0][2], Square::Puttable(vec![
            ReversibleCandidates::new(Position{x: 0, y: 0}, vec![Position{x: 0, y: 1}]),
        ]));
    }
}
