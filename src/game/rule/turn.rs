use yew::Properties;

use super::color::Color;

/// A struct that represents the turn of play in a game.
#[derive(Debug, Clone, Copy, Properties, PartialEq)]
pub struct Turn{
    pub color: Color,
}

impl Turn {
    pub fn change(&self) -> Self {
        match &self.color {
            Color::Black => Turn{color: Color::White},
            Color::White => Turn{color: Color::Black},
        }
    }
}