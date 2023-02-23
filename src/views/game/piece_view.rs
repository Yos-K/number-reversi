
use yew::prelude::*;
use yew::virtual_dom::VNode;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::{prelude::*, JsCast};

use crate::game::rule::color::Color;

#[derive(Debug, Properties, Clone, PartialEq, Copy)]
pub struct PieceViewProps {
    pub color: Color,
    pub value: u8,
    pub num: u8,
}


pub struct PieceView {
    canvas: Option<HtmlCanvasElement>,
    context: Option<CanvasRenderingContext2d>,
    piece_props: PieceViewProps,
}

impl Component for PieceView {
    type Message = ();
    type Properties = PieceViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        let piece = ctx.props();
        Self {
            canvas: None,
            context: None,
            piece_props: *piece,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("apiece").unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
            self.canvas = Some(canvas.clone());
            self.context = Some(canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap());
        }
        let context = self.context.as_ref().unwrap();

        let square_size = 50.0;
        let x = 35.0;
        let y = 25.0;

        context.begin_path();
        context.clear_rect(x, y, 50.0, 50.0);
        match ctx.props().color {
            Color::Black => {
                context.set_fill_style(&JsValue::from_str("black"));
            },
            Color::White => {
                context.set_fill_style(&JsValue::from_str("white"));
            },
        }
        context.arc(x, y, square_size / 2.0 - 5.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        context.fill();
        context.stroke();
        match ctx.props().color {
            Color::Black => {
                context.set_fill_style(&JsValue::from_str("white"));
            },
            Color::White => {
                context.set_fill_style(&JsValue::from_str("black"));
            },
        }
        
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.set_font(&format!("bold 25px serif"));
        context.fill_text(&ctx.props().value.to_string(), x, y);
        context.set_fill_style(&JsValue::from_str("black"));
        context.set_text_align("left");
        context.set_text_baseline("top");
        context.set_font(&format!("bold 20px serif"));
        context.fill_text(&format!("×{}", &ctx.props().num.to_string()), x+20.0, y+2.0);
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> VNode {
        html! {
            <div>
                <canvas id="apiece" width="100" height="50" ></canvas>
            </div>
        }
    }
}
