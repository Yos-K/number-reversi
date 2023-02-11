use game::assets::board::Board;
use game::assets::piece::Piece;
use game::assets::pieces::Pieces;
use game::rule::color::Color;
use game::rule::position::Position;
use game::events::put_piece::put_piece;
use game::rule::turn::Turn;
use game::events::check_puttable_position_exists::check_puttable_position_exists;
use game::assets::piece::UsedPiece;
use views::game::board_view::BoardViewProps;
use views::game::piece_view::PieceViewProps;
use views::game::increment_button::IncrementProps;
use views::game::decrement_button::DecrementProps;
use views::game::piece_view::PieceView;
use yew::prelude::*;
use views::page::header::Header;
use views::game::board_view::BoardView;
use views::game::score_view::ScoreView;
use views::game::decrement_button::DecrementButton;
use views::game::increment_button::IncrementButton;

mod controller;
mod game;
mod views;
mod tests;


#[function_component]
fn App() -> Html {
    let turn = use_state(|| Turn{color: Color::Black});
    let pieces = use_state(|| Pieces::make_pieces());
    let piece = use_state(|| Piece::new(turn.color, 1));
    let used_pieces = use_state(|| UsedPiece::new());
    let position = use_state(|| Position{x: 0, y: 0});
    let rest_num = use_state(|| pieces.get_rest_num(piece.value, *turn));
    let board = use_state(|| check_puttable_position_exists(Board::set_initial_state(), *turn));

    let score = board.get_score();

    let on_decrement = {
        let piece = piece.clone();
        let rest_num = rest_num.clone();
        let pieces = pieces.clone();
        let turn = turn.clone();

        Callback::from(move |current_piece| {
            log::info!("{:?}", current_piece);
            piece.set(current_piece);
            let current_rest_num = pieces.get_rest_num(current_piece.value, *turn);
            rest_num.set(current_rest_num);
            log::info!("{:?}", current_rest_num);
        })
    };

    let on_increment = {
        let piece = piece.clone();
        let rest_num = rest_num.clone();
        let pieces = pieces.clone();
        let turn = turn.clone();

        Callback::from(move |current_piece| {
            log::info!("{:?}", current_piece);
            piece.set(current_piece);
            let current_rest_num = pieces.get_rest_num(current_piece.value, *turn);
            rest_num.set(current_rest_num);
            log::info!("{:?}", current_rest_num);
        })
    };

    let on_put = {
        let position = position.clone();
        let board = board.clone();
        let piece = piece.clone();
        let turn = turn.clone();
        let pieces = pieces.clone();
        let rest_num = rest_num.clone();
        let used_pieces = used_pieces.clone();

        Callback::from(move |current_position| {
            position.set(current_position);
            log::info!("{:?}", current_position);
            let squares = board.get_squares();
            let tmp_board = Board{squares};
            if *rest_num > 0 {
                let (new_board, next_turn, new_pieces, new_piece, new_used_pieces) = 
                    put_piece(current_position, *piece, tmp_board, *turn, (*pieces).clone(), (*used_pieces).clone());
                let current_rest_num = new_pieces.get_rest_num(new_piece.value, next_turn);
                rest_num.set(current_rest_num);
                turn.set(next_turn);
                board.set(new_board);
                piece.set(new_piece);
                pieces.set(new_pieces);
                used_pieces.set(new_used_pieces);
            }
        })
    };

    let decrement_props = DecrementProps {
        piece: *piece,
        on_decrement,
    };

    let increment_props = IncrementProps {
        piece: *piece,
        on_increment,
    };

    log::info!("{:?}", piece);
    let piece_props = PieceViewProps {
        color: (*piece).color,
        value: (*piece).value,
        num: *rest_num,
    };

    let board_props = BoardViewProps {
        squares: board.squares.clone(),
        on_put,
    };

    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
                <div class="d-flex">
                    <BoardView ..board_props />
                    <ScoreView black={score.black} white={score.white} />
                </div>
                <div class="d-flex">
                    <DecrementButton ..decrement_props />
                    <PieceView ..piece_props />
                    <IncrementButton ..increment_props />
                </div>
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}