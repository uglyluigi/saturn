use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct GoogleLoginButton {
    link: ComponentLink<Self>,
}

impl Component for GoogleLoginButton {
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
            <div>
                <div id="g_id_onload"
                    data-client_id="699719776672-56jqfpk1g2uq8tma72hi56n5jkan82nr.apps.googleusercontent.com"
                    data-login_uri="https://ec2-18-189-223-253.us-east-2.compute.amazonaws.com/"
                    data-ux_mode="redirect"
                    data-auto_prompt="false">
                </div>
                <div class="g_id_signin"
                        data-size="large"
                        data-theme="outline"
                        data-text="sign_in_with"
                        data-shape="rectangular"
                        data-logo_alignment="left">
                </div>
            </div>
        }
    }
}