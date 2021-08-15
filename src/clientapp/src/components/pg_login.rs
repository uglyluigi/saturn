use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    Increment,
    Decrement,
}

pub struct LoginPageComponent {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for LoginPageComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                ConsoleService::log("plus one");
                true
            }
            Msg::Decrement => {
                self.value -= 1;
                ConsoleService::log("minus one");
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>

                <div class="auth-header">
                    <img class="saturn-logo" src="assets/saturn-logo.svg" alt="Saturn logo"/>
                    <h1>{"Saturn"}</h1>
                </div>

                <div class="soft-grey-rect">
                    <h1>
                        {"Hello!"}
                    </h1>
                    <hr/>
                    <div class="please-login-text">
                        {"Please log in to proceed."}
                    </div>
                </div>
            </div>
        }
    }
}