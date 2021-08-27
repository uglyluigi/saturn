use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct ToolbarComponent {
    link: ComponentLink<Self>,
}

enum Msg {
    NowActive(String),
}

impl Component for ToolbarComponent {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="toolbar">
                <img src="./assets/saturn-logo.svg"/>
                <div class="links">
                    <a class="active" href="#home">{"Home"}</a>
                    <a href="#smth">{"Something"}</a>
                </div>
            </div>
        }
    }
}
