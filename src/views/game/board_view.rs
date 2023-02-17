
use yew::prelude::*;
use yew::virtual_dom::VNode;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::{prelude::*, JsCast};

use crate::game::{
    assets::{
        board::Board, 
        square::Square
    }, 
    rule::{color::Color, position::Position}
};

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct BoardViewProps {
    pub squares: Vec<Vec<Square>>,
    pub on_put: Callback<Position>,
}

pub struct BoardView {
    board: BoardViewProps,
    canvas: Option<HtmlCanvasElement>,
    context: Option<CanvasRenderingContext2d>,
}

impl Component for BoardView {
    type Message = ();
    type Properties = BoardViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        let board = ctx.props().clone();
        // let on_click = ctx.props().on_put.clone;
        Self {
            canvas: None,
            context: None,
            board,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("aboard").unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
            self.canvas = Some(canvas.clone());
            self.context = Some(canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap());
        }
        
        let context = self.context.as_ref().unwrap();

        context.begin_path();
        context.clear_rect(0.0, 0.0, 400.0, 400.0);
        let square_size = 50.0;
        
        for i in 0..8 {
            for j in 0..8 {
                if (i + j) % 2 == 0 {
                    draw_square(context, i, square_size, j, "#228B22");
                } else {
                    draw_square(context, i, square_size, j, "#90EE90");
                }

                match ctx.props().squares[i][j] {
                    Square::Put(piece) => {
                        let x = (i as f64) * square_size + square_size / 2.0;
                        let y = (j as f64) * square_size + square_size / 2.0;
                        match piece.color {
                            Color::Black => {
                                draw_piece(context, x, y, square_size, "black", piece.value, "white");
                            },
                            Color::White => {
                                draw_piece(context, x, y, square_size, "white", piece.value, "black");
                            },
                        };
                    },
                    Square::Puttable(_) => {
                        draw_square(context, i, square_size, j, "#FFFF90");
                    },
                    _ => (),
                }
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        let square_size = 50.0;
        let on_click = {
            let on_put = ctx.props().on_put.clone();
            Callback::from(move |e: MouseEvent| {
                let x = (e.offset_x() as f64 / square_size).floor() as usize;
                let y = (e.offset_y() as f64 / square_size).floor() as usize;
                let position = Position{
                    x,
                    y
                };
                on_put.emit(position)
            })
        };

        html! {
            <div>
                <canvas id="aboard" width="400" height="400" onclick={on_click} ></canvas>
            </div>
        }
    }
}

fn draw_square(context: &CanvasRenderingContext2d, i: usize, square_size: f64, j: usize, color: &str) {
    context.set_fill_style(&JsValue::from_str(color));
    context.fill_rect(i as f64 * square_size, j as f64 * square_size, square_size, square_size);
    context.rect(i as f64 * square_size, j as f64 * square_size, square_size, square_size);
    context.stroke();
}

fn draw_piece(context: &CanvasRenderingContext2d, x: f64, y: f64, square_size: f64, piece_color: &str, value: u8, font_color: &str) {
    context.begin_path();
    context.set_fill_style(&JsValue::from_str(piece_color));
    context.arc(x, y, square_size / 2.0 - 5.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
    context.fill();
    context.stroke();
    context.set_fill_style(&JsValue::from_str(font_color));
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_font(&format!("bold 25px serif"));
    context.fill_text(&value.to_string(), x, y);
}
