use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct ToolbarComponent {
    link: ComponentLink<Self>,
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
            <span id="toolbar">
                <img src="assets/saturn-logo.svg" alt="Saturn logo"/>
                <h2>{"Saturn"}</h2>
            </span>
        }
    }
}