use crate::components::google_login_button::GoogleLoginButton;
use crate::components::three::ThreeJSViewport;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct LoginPageComponent {
    link: ComponentLink<Self>,
}

impl Component for LoginPageComponent {
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
            <div class="login-page">
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
                        <GoogleLoginButton/>
                    </div>
                </div>
            </div>
        }
    }
}
