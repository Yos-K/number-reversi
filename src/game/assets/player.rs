use std::rc::Rc;

use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Human,
    Com,
}

impl Reducible for Player {
    type Action = Player;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }
}