use yew::{html, Component, ComponentLink, Html, MouseEvent, ShouldRender};

use crate::components::{
	core::{footer::*, router::*},
	GoogleLoginButton,
};

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
		Self {
			link,
			skip_login: false,
		}
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
		let _skip_login = self.link.callback(|_: MouseEvent| Msg::BypassLogin);

		html! {
			<div class="login-page">
				<canvas id="login-canvas"></canvas>

				<div class="login-hero">
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
						</div>
					</div>
				</div>
				<div class="login-content-wrapper">
					<div class="login-content-border"></div>
					<div class="login-content">
						<div class="login-2-panel-split">
							<div class="panel">
								<h1> {"Find "}<span class="text-accent"> {"Your "}</span> {"Communities!"}</h1>
							</div>
							<div class="panel">
								<img src="https://images.pexels.com/photos/933964/pexels-photo-933964.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940"/>
							</div>
						</div>
						<div class="login-2-panel-split">
							<div class="panel">
								<h2> {"Saturn is the platform that lets you "}<span class="text-accent"> {"overcome networking boundaries "}</span> {"hindering your club's growth and gives you the tools to "}<span class="text-accent"> {"connect with others"}</span> {"."}</h2>
							</div>
							<div class="panel">
								<img class="panel-contain" src="/assets/saturn-space.png"/>
							</div>
						</div>
					</div>
				</div>
				<Footer/>
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

				<script defer=true src="bg.js"/>
			</div>
		}
	}
}
