use yew::MouseEvent;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::components::GoogleLoginButton;
use crate::components::ClubView;
use crate::components::core::router::*;
use crate::flags::IS_DEBUG_MODE;

pub struct LoginPageComponent {
	link: ComponentLink<Self>,
	skip_login: bool,
}

pub enum Msg {
	BypassLogin,
}

impl Component for LoginPageComponent {
	type Message = Msg;
	type Properties = ();

	fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link, skip_login: false }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::BypassLogin => {
				self.skip_login = true;
			}
		};

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		let skip_login = self.link.callback(|_:MouseEvent| {
			Msg::BypassLogin
		});

		html! {
			<div class="login-page">
				<canvas height="100" width="100" id="login-canvas"></canvas>
				<script defer=true src="bg.js"/>

				<div class="auth-header">
					<img class="saturn-logo" src="assets/saturn-logo.svg" alt="Saturn logo"/>
					<h1>{"Saturn"}</h1>
				</div>

				<div class="soft-grey-rect">
					<h1>
						{"Welcome to Saturn!"}
					</h1>
					<div class="please-login-text">
						{"Please log in to proceed."}

						<GoogleLoginButton/>
						{
							if *IS_DEBUG_MODE {
								html! {
									<button onclick=skip_login>{"Skip login"}</button>
								}
							} else {
								html! {
									<></>
								}
							}
						}
					</div>
				</div>

				{
					if self.skip_login {
						html! {
							<AppRedirect route=AppRoute::Home/>
						}
					} else {
						html! {
							<></>
						}
					}
				}
			</div>
		}
	}
}
