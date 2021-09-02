use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};

pub struct ToolbarComponent {
    link: ComponentLink<Self>,
    props: Props,
}

enum Msg {
    NowActive(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub username: String,
}

impl Component for ToolbarComponent {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
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
                <img src="./assets/saturn-text-logo.PNG"/>
                <h1>{"Hi, "} {self.props.username.clone()}</h1>
                <div class="links">
                    <a class="active" href="/">{"Home"}</a>
                    <a href="#smth">{"Something"}</a>
                </div>
            </div>
        }
    }
}
