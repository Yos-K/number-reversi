use super::piece::Piece;
use crate::game::rule::position::ReversibleCandidates;

/// Represents a square on a board.
#[derive(Debug, Clone, PartialEq)]
pub enum Square {
    /// The square is empty and no piece has been placed on it.
    Empty,
    /// The square has a placed piece.
    Put(Piece),
    /// The square can have a piece placed on it and also includes a list of reversible candidates.
    Puttable(Vec<ReversibleCandidates>),
}