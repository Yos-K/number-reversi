use crate::game::{assets::{board::Board, square::Square, piece::Piece}, rule::color::Color};

pub struct BoardHelper;

impl BoardHelper {
    pub fn make_board(black_positions: Vec<(usize, usize, u8)>, white_positions: Vec<(usize, usize, u8)>) -> Board {
        let mut squares = vec![vec![Square::Empty; 8]; 8];
        black_positions.iter().for_each(
            |(x, y, n)| squares[*x][*y] = Square::Putted(Piece::new(Color::Black, *n))
        );
        white_positions.iter().for_each(
            |(x, y, n)| squares[*x][*y] = Square::Putted(Piece::new(Color::White, *n))
        );
        Board{squares}
    }
}

#[test]
fn make_board_test() {
    let board = BoardHelper::make_board(vec![(1, 1, 1)], vec![]);
    assert_eq!(board.squares[1][1], Square::Putted(Piece::new(Color::Black, 1)));
}