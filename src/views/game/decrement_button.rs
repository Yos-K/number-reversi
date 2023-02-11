use yew::prelude::*;
use yew::{Callback, function_component, html, Html, Properties};

use crate::game::assets::piece::Piece;

#[derive(Properties, PartialEq)]
pub struct DecrementProps {
    pub piece: Piece,
    pub on_decrement: Callback<Piece>
}

#[function_component(DecrementButton)]
pub fn decrement_button(props: &DecrementProps) -> Html {
    let decrement = {
        let on_decrement = props.on_decrement.clone();
        let selected = props.piece.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let current_piece = match selected.value {
                1 => Piece::new(selected.color, 10),
                _ => Piece::new(selected.color, selected.value - 1)
            };
            on_decrement.emit(current_piece)
        })
    };

    html! {
        <button type="submit" onclick={decrement}>{"◁"}</button>
    }
}