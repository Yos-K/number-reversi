use std::collections::HashSet;

use crate::game::rule::{color::Color, turn::Turn};

/// Represents a Piece on the game board.
/// A Piece has a color (Black or White) and a value(1-10).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub value: u8,
}
impl Piece {
    /// Creates a new `Piece` instance.
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the piece. Either `Color::Black` or `Color::White`.
    /// * `n` - The value of the piece. The range is 1-10.
    ///
    /// # Examples
    ///
    /// ```
    /// use number-reversi::game::assets::piece::Piece;
    /// use number-reversi::game::rule::color::Color;
    /// 
    /// let black_piece = Piece::new(Color::Black, 1);
    /// assert_eq!(black_piece.color, Color::Black);
    /// assert_eq!(black_piece.value, 1);
    /// 
    /// let white_piece = Piece::new(Color::White, 10);
    /// assert_eq!(white_piece.color, Color::White);
    /// assert_eq!(white_piece.value, 10);
    /// ```
    pub fn new(color: Color, n: u8) -> Self {
        Self{
            color,
            value: n
        }
    }
    /// Reverses the color of the piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use number-reversi::game::assets::piece::Piece;
    /// use number-reversi::game::rule::color::Color;
    /// 
    /// let black = Piece::new(Color::Black, 1);
    /// assert_eq!(black.reverse(), Piece::new(Color::White, 1));
    ///
    /// let white = Piece::new(Color::White, 2);
    /// assert_eq!(white.reverse(), Piece::new(Color::Black, 2));
    /// ```
    pub fn reverse(&self) -> Self {
        match &self.color {
            Color::Black => Piece::new(Color::White, self.value),
            Color::White => Piece::new(Color::Black, self.value),
        }
    }
}


/// Represents the set of `Piece` instances that have been used in the game.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UsedPiece{
    pub piece_set: HashSet<Piece>
}

impl UsedPiece {
    /// Creates a new empty `UsedPiece`.
    ///
    /// # Examples
    ///
    /// ```
    /// use number-reversi::game::assets::piece::UsedPiece;
    /// 
    /// let used_piece = UsedPiece::new();
    /// assert!(used_piece.piece_set.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { piece_set: HashSet::new() }
    }

    /// Adds a `Piece` to the `UsedPiece`.
    ///
    /// # Examples
    ///
    /// ```
    /// use number-reversi::game::assets::piece::{UsedPiece, Piece};
    /// use number-reversi::game::rule::color::Color;
    /// 
    /// let piece = Piece::new(Color::Black, 1);
    /// let mut used_piece = UsedPiece::new();
    /// used_piece = used_piece.add_piece(piece);
    /// assert!(used_piece.piece_set.contains(&piece));
    /// ```
    pub fn add_piece(&self, piece: Piece) -> Self {
        let mut new_set = self.piece_set.clone();
        new_set.insert(piece);
        Self { piece_set: new_set }
    }

    /// Removes a `Piece` from the `UsedPiece`.
    ///
    /// # Examples
    ///
    /// ```
    /// use number-reversi::game::assets::piece::{UsedPiece, Piece};
    /// use number-reversi::game::rule::color::Color;
    /// 
    /// let piece = Piece::new(Color::Black, 1);
    /// let mut used_piece = UsedPiece::new().add_piece(piece);
    /// used_piece = used_piece.remove_piece(piece);
    /// assert!(!used_piece.piece_set.contains(&piece));
    /// ```
    pub fn remove_piece(&self, piece: Piece) -> Self {
        let mut new_set = self.piece_set.clone();
        new_set.remove(&piece);
        Self { piece_set: new_set }
    }

    /// Gets the piece that the opponent will receive when passing.
    /// The piece selected is the highest-valued piece of the opponent color in the `piece_set`.
    ///
    /// # Examples
    ///
    /// ```
    /// use number-reversi::game::assets::piece::{UsedPiece, Piece};
    /// use number-reversi::game::rule::{color::Color, turn::Turn};
    /// 
    /// let mut used_piece = UsedPiece::new();
    /// used_piece = used_piece.add_piece(Piece::new(Color::Black, 1));
    /// used_piece = used_piece.add_piece(Piece::new(Color::Black, 3));
    /// 
    /// let turn = Turn{color: Color::Black};
    /// assert_eq!(used_piece.get_add_piece(turn), Piece::new(Color::Black, 3));
    /// ```
    pub fn get_add_piece(&self, turn: Turn) -> Piece {
        self.piece_set.iter().filter(|p| p.color == turn.color)
        .max_by_key(|p| p.value).unwrap().clone()
    }
}


#[cfg(test)]
mod piece_test {
    mod piece_property_test {
        use crate::game::{
            assets::piece::Piece, 
            rule::color::Color
        };
        #[test]
        fn piece_has_color_and_num_property(){
            assert_eq!(Piece::new(Color::Black, 1), Piece::new(Color::Black, 1));
        }
    }
    #[cfg(test)]
    mod reverse_methods_test {
        use crate::game::{
            assets::piece::Piece, 
            rule::color::Color
        };
        #[test]
        fn when_piece_black1_then_returns_piece_white1() {
            let black = Piece::new(Color::Black, 1);
            assert_eq!(black.reverse(), Piece::new(Color::White, 1));
        }
        #[test]
        fn when_piece_white2_then_returns_piece_black2() {
            let black = Piece::new(Color::White, 2);
            assert_eq!(black.reverse(), Piece::new(Color::Black, 2));
        }
    }
}