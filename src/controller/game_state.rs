use crate::game::rule::turn::Turn;
use yew::UseStateHandle;

use crate::game::assets::{
    board::Board, 
    piece::Piece, 
    pieces::Pieces
};


#[derive(Debug, Clone)]
pub struct GameState {
    pub board: UseStateHandle<Board>,
    pub piece: UseStateHandle<Piece>,
    pub pieces: UseStateHandle<Pieces>,
    pub turn: UseStateHandle<Turn>,
}