use yew::{html, services::ConsoleService, Component, ComponentLink, Html, ShouldRender};

pub struct GoogleLoginButton {
	link: ComponentLink<Self>,
	url: String,
}

pub enum Msg {
	GooglePLLoaded,
}

impl Component for GoogleLoginButton {
	type Message = Msg;
	type Properties = ();

	fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			url: String::from(match std::env::var("SATURN_EXEC_PROFILE") {
				Ok(val) => match val.as_str() {
					"LOCAL" => "localhost:8080/api/auth/login",
					_ => "https://joinsaturn.net/api/auth/login",
				},

				Err(_) => "https://joinsaturn.net/api/auth/login",
			}),
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::GooglePLLoaded => true,
			_ => false,
		}
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div>
				<script id="gplib_script" src="https://accounts.google.com/gsi/client"></script>

				<div id="google-button-container">
					<div id="g_id_onload"
						data-client_id="699719776672-56jqfpk1g2uq8tma72hi56n5jkan82nr.apps.googleusercontent.com"
						data-login_uri=self.url.clone()
						data-ux_mode="redirect"
						data-auto_prompt="false">
					</div>
					<div class="g_id_signin"
							data-size="large"
							data-theme="filled_black"
							data-text="sign_in_with"
							data-shape="circle"
							data-logo_alignment="left">
					</div>
				</div>
			</div>
		}
	}
}
