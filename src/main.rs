mod components;
use crate::components::minesweeper::GameBoard;
use components::minesweeper::GameBoardProps;
use yew::prelude::*;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = GameBoardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        print!("update called");
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <GameBoard rows=10 cols=10 />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}