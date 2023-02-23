use yew::prelude::*;
use crate::game::assets::board::Board;
use crate::game::assets::piece::Piece;
use crate::game::assets::pieces::Pieces;
use crate::game::assets::player::Player;
use crate::game::rule::color::Color;
use crate::game::rule::position::Position;
use crate::game::events::put_piece::put_piece;
use crate::game::rule::turn::Turn;
use crate::game::events::check_puttable_position_exists::check_puttable_position_exists;
use crate::game::assets::piece::UsedPiece;
use crate::views::game::board_view::BoardViewProps;
use crate::views::game::piece_view::PieceViewProps;
use crate::views::game::increment_button::IncrementProps;
use crate::views::game::decrement_button::DecrementProps;
use crate::views::game::piece_view::PieceView;
use crate::views::game::board_view::BoardView;
use crate::views::game::score_view::ScoreView;
use crate::views::game::decrement_button::DecrementButton;
use crate::views::game::increment_button::IncrementButton;

#[derive(Debug, Properties, PartialEq, Clone, Copy)]
pub struct GameModeProps{
    pub black: Player,
    pub white: Player,
}

#[function_component]
pub fn Game(props: &GameModeProps) -> Html {
    let turn = use_state(|| Turn{color: Color::Black, player: props.black});
    let pieces = use_state(|| Pieces::make_pieces());
    let piece = use_state(|| Piece::new(turn.color, 1));
    let used_pieces = use_state(|| UsedPiece::new());
    let position = use_state(|| Position{x: 0, y: 0});
    let rest_num = use_state(|| pieces.get_rest_num(piece.value, *turn));
    let board = use_state(|| check_puttable_position_exists(Board::set_initial_state(), *turn));

    let score = board.get_score();

    if (*turn).player == Player::Com{
        log::info!("com's turn");
        let pos = board.get_puttable_position();
        let selected_piece = pieces.select_piece(*turn);

        let squares = board.get_squares();
        let tmp_board = Board{squares};

        let (new_board, next_turn, new_pieces, new_piece, new_used_pieces) = 
            put_piece(pos, selected_piece, tmp_board, *turn, (*pieces).clone(), (*used_pieces).clone(), *props);
        let current_rest_num = new_pieces.get_rest_num(new_piece.value, next_turn);
        rest_num.set(current_rest_num);
        turn.set(next_turn);
        board.set(new_board);
        piece.set(new_piece);
        pieces.set(new_pieces);
        used_pieces.set(new_used_pieces);
    };

    let on_decrement = {
        let piece = piece.clone();
        let rest_num = rest_num.clone();
        let pieces = pieces.clone();
        let turn = turn.clone();

        Callback::from(move |current_piece| {
            piece.set(current_piece);
            let current_rest_num = pieces.get_rest_num(current_piece.value, *turn);
            rest_num.set(current_rest_num);
        })
    };

    let on_increment = {
        let piece = piece.clone();
        let rest_num = rest_num.clone();
        let pieces = pieces.clone();
        let turn = turn.clone();

        Callback::from(move |current_piece| {
            piece.set(current_piece);
            let current_rest_num = pieces.get_rest_num(current_piece.value, *turn);
            rest_num.set(current_rest_num);
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
        let game_mode_props = props.clone();

        match (*turn).player {
            Player::Human => {
                Callback::from(move |current_position| {
                    position.set(current_position);
                    let squares = board.get_squares();
                    let tmp_board = Board{squares};
                    if *rest_num > 0 {
                        let (new_board, next_turn, new_pieces, new_piece, new_used_pieces) = 
                            put_piece(current_position, *piece, tmp_board, *turn, (*pieces).clone(), (*used_pieces).clone(), game_mode_props);
                        let current_rest_num = new_pieces.get_rest_num(new_piece.value, next_turn);
                        rest_num.set(current_rest_num);
                        turn.set(next_turn);
                        board.set(new_board);
                        piece.set(new_piece);
                        pieces.set(new_pieces);
                        used_pieces.set(new_used_pieces);
                    }
                })
            },
            Player::Com => {
                Callback::from(move |_| {})
            },
        }
    };

    let decrement_props = DecrementProps {
        piece: *piece,
        on_decrement,
    };

    let increment_props = IncrementProps {
        piece: *piece,
        on_increment,
    };

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