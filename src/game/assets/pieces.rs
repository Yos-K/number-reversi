use rand::Rng;
use std::collections::HashMap;

use crate::game::rule::{color::Color, turn::Turn};

use super::piece::Piece;

/// Pieces represents the remaining pieces of a player.
#[derive(Debug, PartialEq, Clone)]
pub struct Pieces {
    /// the number of remaining black pieces, represented as a HashMap 
    /// from piece value to the number of pieces with that value
    pub black: HashMap<u8, u8>,
    /// the number of remaining white pieces, represented as a HashMap 
    /// from piece value to the number of pieces with that value
    pub white: HashMap<u8, u8>,
}
impl Pieces {
    /// make_pieces creates an initial set of pieces for both black and white.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use number-reversi::game::assets::pieces::Pieces;
    /// 
    /// let pieces = Pieces::make_pieces();
    /// assert_eq!(pieces.black.get(&1), Some(&5));
    /// assert_eq!(pieces.black.get(&10), Some(&1));
    /// assert_eq!(pieces.white.get(&1), Some(&5));
    /// assert_eq!(pieces.white.get(&10), Some(&1));
    /// ```
    pub fn make_pieces() -> Self {
        let piece_set = (1..=10).map(|i| {
            let piece_num = match i {
                1 | 2 => 5,
                3 | 4 => 4,
                5 | 6 => 3,
                7 | 8 => 2,
                _ => 1
            };
            (i, piece_num)
        }).collect::<HashMap<u8, u8>>();
        let other_hand = piece_set.clone();
        Self { 
            black: piece_set,
            white: other_hand,
        }
    }
    /// remove a piece, and return a new Pieces instance.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use number-reversi::game::assets::piece::Piece;
    /// use number-reversi::game::assets::pieces::Pieces;
    /// use number-reversi::game::rule::color::Color;
    /// 
    /// // remove a black piece1
    /// let pieces = Pieces::make_pieces();
    /// let pieces = pieces.remove(&Piece::new(Color::Black, 1));
    /// assert_eq!(pieces.black.get(&1), Some(&4));
    /// ```
    pub fn remove(&self, piece: &Piece) -> Self {
        match piece.color {
            Color::Black => {
                let mut piece_set = self.black.clone();
                let piece_num = piece_set.get_mut(&piece.value).unwrap();
                *piece_num -= 1;
                Self { black: piece_set, white: self.white.clone() }
            },
            Color::White => {
                let mut piece_set = self.white.clone();
                let piece_num = piece_set.get_mut(&piece.value).unwrap();
                *piece_num -= 1;
                Self { black: self.black.clone(), white: piece_set }
            },
        }
    }
    /// add a piece, and return a new Pieces instance.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use number-reversi::game::assets::piece::Piece;
    /// use number-reversi::game::assets::pieces::Pieces;
    /// use number-reversi::game::rule::color::Color;
    /// 
    /// // add a black piece1
    /// let pieces = Pieces::make_pieces();
    /// let pieces = pieces.add(&Piece::new(Color::Black, 1));
    /// assert_eq!(pieces.black.get(&1), Some(&6));
    /// ```
    pub fn add(&self, piece: &Piece) -> Self {
        match piece.color {
            Color::Black => {
                let mut piece_set = self.black.clone();
                let piece_num = piece_set.get_mut(&piece.value).unwrap();
                *piece_num += 1;
                Self { black: piece_set, white: self.white.clone() }
            },
            Color::White => {
                let mut piece_set = self.white.clone();
                let piece_num = piece_set.get_mut(&piece.value).unwrap();
                *piece_num += 1;
                Self { black: self.black.clone(), white: piece_set }
            },
        }
    }

    /// get_rest_num returns the number of remaining pieces of the specified value and turn.
    /// 
    /// # Arguments
    /// 
    /// * `piece_value` - the value of a piece. (1 <= piece_value <= 10)
    /// * `turn` - the turn of the player who has the pieces.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use number-reversi::game::assets::piece::Piece;
    /// use number-reversi::game::assets::pieces::Pieces;
    /// use number-reversi::game::rule::{color::Color, turn::Turn};
    /// 
    /// let pieces = Pieces::make_pieces();
    /// let rest_num = pieces.get_rest_num(1, Turn{color: Color::Black});
    /// assert_eq!(rest_num, 5);
    /// ```
    pub fn get_rest_num(&self, piece_value: u8, turn: Turn) -> u8 {
        match turn.color {
            Color::Black => *self.black.get(&piece_value).unwrap(),
            Color::White => *self.white.get(&piece_value).unwrap(),
        }
    }

    pub fn select_piece(&self, turn: Turn) -> Piece {
        let pieces = match turn.color {
            Color::Black => self.black.clone(),
            Color::White => self.white.clone(),
        };
        let pieces: Vec<(&u8, &u8)> = pieces.iter()
            .filter(|v| v.1 > &0)
            .collect();
        let i = rand::thread_rng().gen_range(0..pieces.len());
        let value = *(pieces[i].0);
        Piece { color: turn.color, value }
    }
}




#[cfg(test)]
mod pieces_test {
    #[cfg(test)]
    mod pieces_property_test {}
    #[cfg(test)]
    mod make_pieces_test {
        use crate::game::assets::pieces::Pieces;
        #[test]
        fn make_pieces_creates_correct_number_of_each_black_pieces() {
            let pieces = Pieces::make_pieces();
            assert_eq!(pieces.black.get(&1), Some(&5));
            assert_eq!(pieces.black.get(&2), Some(&5));
            assert_eq!(pieces.black.get(&3), Some(&4));
            assert_eq!(pieces.black.get(&4), Some(&4));
            assert_eq!(pieces.black.get(&5), Some(&3));
            assert_eq!(pieces.black.get(&6), Some(&3));
            assert_eq!(pieces.black.get(&7), Some(&2));
            assert_eq!(pieces.black.get(&8), Some(&2));
            assert_eq!(pieces.black.get(&9), Some(&1));
            assert_eq!(pieces.black.get(&10), Some(&1));
            assert_eq!(pieces.white.get(&1), Some(&5));
            assert_eq!(pieces.white.get(&2), Some(&5));
            assert_eq!(pieces.white.get(&3), Some(&4));
            assert_eq!(pieces.white.get(&4), Some(&4));
            assert_eq!(pieces.white.get(&5), Some(&3));
            assert_eq!(pieces.white.get(&6), Some(&3));
            assert_eq!(pieces.white.get(&7), Some(&2));
            assert_eq!(pieces.white.get(&8), Some(&2));
            assert_eq!(pieces.white.get(&9), Some(&1));
            assert_eq!(pieces.white.get(&10), Some(&1));
        }
    }
    mod remove_test {
        use crate::game::{
            assets::{
                piece::Piece, 
                pieces::Pieces
            }, 
            rule::color::Color
        };
        #[test]
        fn when_put_a_piece_black1_then_num_of_piece_black1_is_4() {
            let put_piece = Piece::new(Color::Black, 1);
            let pieces = Pieces::make_pieces();
            let pieces = pieces.remove(&put_piece);
            assert_eq!(pieces.black.get(&1), Some(&4));
        }
    }
    mod add_test {
        use crate::game::{
            assets::{
                piece::Piece, 
                pieces::Pieces
            }, 
            rule::color::Color
        };
        #[test]
        fn when_white_passed_then_black_max_piece_increase() {
            let put_piece = Piece::new(Color::Black, 1);
            let pieces = Pieces::make_pieces();
            let pieces = pieces.remove(&put_piece);
            let pieces = pieces.add(&put_piece);
            assert_eq!(pieces.black.get(&1), Some(&5));
        }
    }
}