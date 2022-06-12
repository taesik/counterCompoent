use yew::prelude::*;

enum Msg {
    AddOne
}

struct CounterComponent {
    count : i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx:&Context<Self>) -> Self {
        Self { count:0}
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true // rerender component
            }

        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="container">
                <h1>{ self.count }</h1>
                <button onclick={link.callback(|_| Msg::AddOne)}>{"Add One"}</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}