use yew::prelude::*;
use yew::{Callback, function_component, html, Html, Properties};

use crate::app::game::GameModeProps;
use crate::game::assets::player::Player;

use super::game::Game;

#[derive(Properties, PartialEq, Clone)]
pub struct PlayerProps {
    pub on_select_black: Callback<Player>,
    pub on_select_white: Callback<Player>,
}

#[function_component(Menu)]
pub fn menu_page() -> Html {
    let player_black = use_state(|| Player::Human);
    let player_white = use_state(|| Player::Human);

    let is_start = use_state(|| false);

    let black = player_black.clone();
    let white = player_white.clone();

    let game_mode = GameModeProps {
        black: *black,
        white: *white
    };

    let select_human_black_side = {
        let player_black = player_black.clone();
        Callback::from(move |e| {
            player_black.set(Player::Human);
        })
    };

    let select_com_black_side = {
        let player_black = player_black.clone();
        Callback::from(move |e| {
            player_black.set(Player::Com);
        })
    };

    let select_human_white_side = {
        let player_white = player_white.clone();
        Callback::from(move |e| {
            player_white.set(Player::Human);
        })
    };

    let select_com_white_side = {
        let player_white = player_white.clone();
        Callback::from(move |e: MouseEvent| {
            player_white.set(Player::Com);
        })
    };

    let game_start = {
        let is_start = is_start.clone();
        Callback::from(move |e: MouseEvent| {
            is_start.set(true);
        })
    };

    if *is_start {
        return html!{<Game ..game_mode />}
    } else {
        return html! {
            <>
                <form>
                    <fieldset>
                        <legend>{"Black"}</legend>
                        <input 
                            type="radio" 
                            id="human" 
                            name="gameMode" 
                            value="human" 
                            onclick={select_human_black_side}
                        />
                        <label for="practice">{"Human"}</label>
                        <input 
                            type="radio" 
                            id="com" 
                            name="gameMode" 
                            value="com" 
                            onclick={select_com_black_side}
                        />
                        <label for="vs">{"COM"}</label>
                    </fieldset>
                </form>
                <form>
                    <fieldset>
                        <legend>{"White"}</legend>
                        <input 
                            type="radio" 
                            id="human" 
                            name="gameMode" 
                            value="human" 
                            onclick={select_human_white_side}
                        />
                        <label for="practice">{"Human"}</label>
                        <input 
                            type="radio" 
                            id="com" 
                            name="gameMode" 
                            value="com" 
                            onclick={select_com_white_side}
                        />
                        <label for="vs">{"COM"}</label>
                    </fieldset>
                </form>
                <button onclick={game_start}>{"Game Start!"}</button>
            </>
        }
    }
}