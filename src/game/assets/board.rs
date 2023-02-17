//! `Board` is a struct to represent the state of the board in Othello.
//! 
//! # Examples
//! 
//! ```
//! use number-reversi::game::rule::{
//!     color::Color, 
//!     position::Position,
//!     score::Score
//! };
//! use number-reversi::game::assets::{board::Board, square::Square, piece::Piece};
//! 
//! let board = Board::set_initial_state();
//! 
//! assert_eq!(board.get_score(), Score { black: 2, white: 2 });
//! ```
//! 
//! # Methods
//! 
//! * `set_initial_state` - returns a new `Board` instance with the initial state of the game
//! * `is_put` - returns a boolean indicating whether the piece is put at the specified `Position`
//! * `put_piece` - returns a new `Board` instance with the piece put at the specified `Position`
//! * `get_squares` - returns a copy of all `Square`s in the board
//! * `get_score` - returns the `Score` of black and white pieces
//! * `has_puttable` - returns a boolean indicating whether any `Square` is `Puttable`

use crate::game::rule::{
    color::Color, 
    position::Position,
    score::Score
};

use super::{square::Square, piece::Piece};

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub squares: Vec<Vec<Square>>,
}
impl Board {
    /// Creates a new `Board` with the initial state of the game.
    /// The initial state is defined as follows:
    /// - The black pieces are placed in the center of the board, in positions (3, 3) and (4, 4)
    /// - The white pieces are placed in the center of the board, in positions (3, 4) and (4, 3)
    ///
    /// # Examples
    ///
    /// ```
    /// use number-reversi::game::assets::board::Board;
    ///
    /// let board = Board::set_initial_state();
    /// ```
    pub fn set_initial_state() -> Self {
        
        let mut board = Self {
            squares: vec![vec![Square::Empty; 8]; 8]
        };
        board.squares[3][3] = Square::Put(Piece::new(Color::Black, 1));
        board.squares[4][4] = Square::Put(Piece::new(Color::Black, 1));
        board.squares[3][4] = Square::Put(Piece::new(Color::White, 1));
        board.squares[4][3] = Square::Put(Piece::new(Color::White, 1));
        board
    }

    /// Returns a boolean indicating whether a `Piece` has been put in a certain `Position`.
    ///
    /// # Arguments
    ///
    /// * `position` - The `Position` to check.
    ///
    /// # Examples
    ///
    /// ```
    /// use number-reversi::game::assets::board::Board;
    /// use number-reversi::game::rule::position::Position;
    ///
    /// let board = Board::set_initial_state();
    /// let position = Position{x: 3, y: 3};
    ///
    /// assert!(board.is_put(position));
    /// ```
    pub fn is_put(&self, position: Position) -> bool {
        match self.squares[position.x][position.y] {
            Square::Put(_) => true,
            _ => false,
        }
    }

    /// Returns a new `Board` after putting a `Piece` in a certain `Position`.
    ///
    /// # Arguments
    ///
    /// * `position` - The `Position` to put the `Piece`.
    /// * `piece` - The `Piece` to be put.
    ///
    /// # Examples
    ///
    /// ```
    /// use number-reversi::game::assets::{board::Board, piece::Piece};
    /// use number-reversi::game::rule::position::Position;
    /// use number-reversi::game::rule::color::Color;
    ///
    /// let board = Board::set_initial_state();
    /// let position = Position{x: 0, y: 0};
    /// let piece = Piece::new(Color::Black, 1);
    ///
    /// let new_board = board.put_piece(position, piece);
    /// ```
    pub fn put_piece(&self, position: Position, piece: Piece) -> Self {
        let squares = self.squares.clone();
        let mut board = Self {
            squares
        };
        board.squares[position.x][position.y] = Square::Put(piece);
        board
    }

    /// Get a 2D vector of squares representing the board state.
    ///
    /// # Example
    ///
    /// ```
    /// use number-reversi::game::assets::{board::Board, square::Square};
    ///
    /// let board = Board::set_initial_state();
    /// let squares = board.get_squares();
    /// assert_eq!(squares[0][0], Square::Empty);
    /// ```
    pub fn get_squares(&self) -> Vec<Vec<Square>> {
        self.squares.clone()
    }

    fn calculate_score(&self, color: Color) -> u8 {
        self.squares
        .iter()
        .flatten()
        .filter_map(|x| {
            match x {
                Square::Put(p) if p.color == color => Some(p.value),
                _ => None,
            }
        })
        .sum()
    }

    /// Get the score of the black and white pieces on the board.
    ///
    /// # Example
    ///
    /// ```
    /// use number-reversi::game::assets::board::Board;
    /// use number-reversi::game::rule::score::Score;
    ///
    /// let board = Board::set_initial_state();
    /// let score = board.get_score();
    /// assert_eq!(score, Score { black: 2, white: 2 });
    /// ```
    pub fn get_score(&self) -> Score {
        let black_score = self.calculate_score(Color::Black);
        let white_score = self.calculate_score(Color::White);

        Score {
            black: black_score as usize,
            white: white_score as usize,
        }
    }

    /// Check if there is any puttable square on the board.
    ///
    /// # Example
    ///
    /// ```
    /// use number-reversi::game::assets::board::Board;
    ///
    /// let board = Board::set_initial_state();
    /// assert_eq!(board.has_puttable(), false);
    /// ```
    pub fn has_puttable(&self) -> bool {
        self.squares.iter().flatten().any(|x| {
            match x {
                Square::Puttable(_) => true,
                _ => false,
            }
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_set_initial_state() {
        let board = Board::set_initial_state();
        
        assert_eq!(board.squares[3][3], Square::Put(Piece::new(Color::Black, 1)));
        assert_eq!(board.squares[4][4], Square::Put(Piece::new(Color::Black, 1)));
        assert_eq!(board.squares[3][4], Square::Put(Piece::new(Color::White, 1)));
        assert_eq!(board.squares[4][3], Square::Put(Piece::new(Color::White, 1)));
    }

    #[test]
    fn is_put_test() {
        let board = Board::set_initial_state();
        
        assert!(board.is_put(Position {x: 3, y: 3}));
        assert!(board.is_put(Position {x: 4, y: 4}));
        assert!(board.is_put(Position {x: 3, y: 4}));
        assert!(board.is_put(Position {x: 4, y: 3}));
        assert_eq!(board.is_put(Position {x: 0, y: 0}), false);
    }

    #[test]
    fn put_piece_test() {
        let board = Board::set_initial_state();
        let piece = Piece::new(Color::Black, 1);
        
        let new_board = board.put_piece(Position {x: 0, y: 0}, piece.clone());
        assert_eq!(new_board.squares[0][0], Square::Put(piece));
    }

    #[test]
    fn get_squares_test() {
        let board = Board::set_initial_state();
        let squares = board.get_squares();
        
        assert_eq!(squares, board.squares);
    }

    #[test]
    fn get_score_test() {
        let board = Board::set_initial_state();
        let score = board.get_score();
        assert_eq!(score.black, 2);
        assert_eq!(score.white, 2);
    }

    #[test]
    fn has_puttable_test() {
        let board = Board::set_initial_state();
        assert_eq!(board.has_puttable(), false);
    }
}
