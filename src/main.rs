use yew::prelude::*;

enum Msg {
    AddOne,
}

struct CounterComponent {
    value: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        CounterComponent { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true  // re-renders the component
            }
        }
    }

    // similar to React's render function
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{ self.value }</p>
                <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "Add one" }</button>
            </div>
        }
    }
}


fn main() {
    yew::start_app::<CounterComponent>();
}
