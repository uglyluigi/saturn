use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use crate::components::core::router::*;

pub struct Footer {
	link: ComponentLink<Self>,
	props: Props,
}

enum Msg {
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

impl Component for Footer {
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
            <div class="footer">
                <div class="footer-section-container">
                    <div class="footer-section">
                        <ul>
                            <li class="footer-section-header">{"Saturn Team"}</li>
                            <li><a href="https://booglejr.com/" target="_blank">{"☄️ Justin Woodring"}</a></li>
                            <li>{"🌲 Brennan Forrest"}</li>
                            <li>{"🐢 Ashlynn Martell"}</li>
                        </ul>
                    </div>
                    <div class="footer-section">
                        <ul>
                            <li class="footer-section-header">{"Extras"}</li>
                            <li>{"☕ Support Us"}</li>
                            <li>{"🔒 Privacy Notices"}</li>
                            <li><a href="mailto:nonexistentemail.com" target="_blank">{"💬 Contact Us"}</a></li>
                        </ul>
                    </div>
                </div>
                <div class="footer-bottom">
                    <h6> {"Copyright © 2021 joinsaturn.net. Made with ❤️. All Rights Reserved."} </h6>
                </div>
            </div>
        }
    }
}
    