use yew::Properties;

use crate::{app::game::GameModeProps, game::assets::player::Player};

use super::color::Color;

/// A struct that represents the turn of play in a game.
#[derive(Debug, Clone, Copy, Properties, PartialEq)]
pub struct Turn{
    pub color: Color,
    pub player: Player,
}

impl Turn {
    pub fn change(&self, game_mode_prop: GameModeProps) -> Self {
        match &self.color {
            Color::Black => Turn{color: Color::White, player: game_mode_prop.white},
            Color::White => Turn{color: Color::Black, player: game_mode_prop.black},
        }
    }
}