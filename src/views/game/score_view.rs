use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsCast;

use crate::game::rule::score::Score;

pub struct ScoreView {
    canvas: Option<HtmlCanvasElement>,
    context: Option<CanvasRenderingContext2d>,
    score: Score,
}

impl Component for ScoreView {
    type Message = ();
    type Properties = Score;

    fn create(ctx: &Context<Self>) -> Self {
        let score = ctx.props();
        Self {
            canvas: None,
            context: None,
            score: *score,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("score").unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
            self.canvas = Some(canvas.clone());
            self.context = Some(canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap());
        }
        let context = self.context.as_ref().unwrap();
        context.begin_path();
        context.clear_rect(0.0, 0.0, 450.0, 100.0);
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.set_font(&format!("bold 25px serif"));
        context.fill_text(&format!("Score"), 50.0, 30.0);
        context.set_font(&format!("bold 20px serif"));
        context.fill_text(&format!("Black:{:?}", &ctx.props().black), 70.0, 55.0);
        context.fill_text(&format!("White:{:?}", &ctx.props().white), 70.0, 80.0);
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <canvas id="score" width="150" height="100" ></canvas>
            </div>
        }
    }
}