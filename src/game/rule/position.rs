use yew::Properties;

/// Position type is a struct that contains x and y coordinates
///  of a position on the board. 
#[derive(Debug, Clone, Copy, PartialEq, Properties, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    /// Creates a new `Position` with the next `(x, y)` value.
    /// 
    /// # Arguments
    /// 
    /// * `d` - A tuple with `x` and `y` offsets.
    /// 
    /// # Example
    /// 
    /// ```
    /// use number-reversi::game::rule::position::Position;
    /// 
    /// let position = Position { x: 3, y: 4 };
    /// let next_position = position.next_position((1, 1));
    /// 
    /// assert_eq!(next_position, (4, 5));
    /// ```
    pub fn next_position(&self, d: (isize, isize)) -> (isize, isize) {
        (self.x as isize + d.0, self.y as isize + d.1)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct ReversibleCandidates {
    pub opposite: Position,
    pub positions: Vec<Position>,
}
impl ReversibleCandidates {
    /// Creates a new `ReversibleCandidates` struct.
    /// 
    /// # Arguments
    /// 
    /// * `opposite` - The `Position` of the opposite piece.
    /// * `positions` - The vector of `Position` that can be reversed.
    /// 
    /// # Example
    /// 
    /// ```
    /// use number-reversi::game::rule::position::{Position, ReversibleCandidates};
    /// 
    /// let opposite = Position { x: 0, y: 0 };
    /// let positions = vec![
    ///     Position { x: 1, y: 1 },
    ///     Position { x: 2, y: 2 },
    /// ];
    /// ```
    pub fn new(opposite: Position, positions: Vec<Position>) -> Self {
        Self { opposite, positions }
    }
}