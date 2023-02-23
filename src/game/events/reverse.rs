use crate::game::{
    assets::{
        board::Board, piece::Piece, 
        square::Square
    }, 
    rule::position::ReversibleCandidates
};

/// Reverses the `Piece`s in the specified positions.
///
/// # Arguments
/// 
/// * `board` - the current state of the game board
/// * `piece` - the `Piece` that was just placed on the board
/// * `candidates` - the positions of the `Piece`s to be reversed
/// 
/// # Returns
/// 
/// Returns the updated state of the game board with the reversed `Piece`s.
pub fn reverse(board: Board, piece: Piece, candidates: Vec<ReversibleCandidates>) -> Board {
    let mut squares = board.squares.clone();

    for c in candidates.into_iter() {
        if let Square::Put(opposite) = board.squares[c.opposite.x][c.opposite.y] {
            let some_of_both_ends = piece.value + opposite.value;
            for p in c.positions.into_iter() {
                if let Square::Put(r) = board.squares[p.x][p.y] {
                    if r.value < some_of_both_ends {
                        squares[p.x][p.y] = Square::Put(r.reverse());
                    }
                }
            }
        };
    }
    Board{squares}
}

#[cfg(test)]
mod reverse_test {
    use crate::game::{assets::{board::Board, piece::Piece, square::Square, player::Player}, events::check_puttable_position_exists::check_puttable_position_exists, rule::{turn::Turn, color::Color, position::{Position, ReversibleCandidates}}};

    use super::reverse;

    #[test]
    fn test() {
        let turn = Turn{color: Color::Black, player: Player::Human};
        let board = check_puttable_position_exists(Board::set_initial_state(), turn);
        let position = Position{x: 4, y: 2};
        let piece = Piece::new(Color::Black, 1);
        let candidates = vec![
            ReversibleCandidates{
                opposite: Position{x: 4, y: 4}, 
                positions: vec![Position{x: 4, y: 3}]}];
        let board = board.put_piece(position, piece);
        let board = reverse(board, piece, candidates);
        assert_eq!(board.squares[4][3], Square::Put(Piece::new(Color::Black, 1)));
    }
}