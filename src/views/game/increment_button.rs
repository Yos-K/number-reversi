use yew::prelude::*;
use yew::{Callback, function_component, html, Html, Properties};

use crate::game::assets::piece::Piece;

#[derive(Properties, PartialEq)]
pub struct IncrementProps {
    pub piece: Piece,
    pub on_increment: Callback<Piece>
}

#[function_component(IncrementButton)]
pub fn increment_button(props: &IncrementProps) -> Html {
    let increment = {
        let on_decrement = props.on_increment.clone();
        let selected = props.piece.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let current_piece = match selected.value {
                10 => Piece::new(selected.color, 1),
                _ => Piece::new(selected.color, selected.value + 1)
            };
            on_decrement.emit(current_piece)
        })
    };

    html! {
        <button type="submit" onclick={increment}>{"▷"}</button>
    }
}