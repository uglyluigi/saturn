use web_sys::MouseEvent;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::components::{ClubView, GoogleLoginButton};

pub struct CreateClubFloatingActionButton {
	link: ComponentLink<Self>,
	props: Props,
}

#[derive(Debug, Properties, Clone)]
pub struct Props {
	pub parent_link: ComponentLink<ClubView>,
}

impl Component for CreateClubFloatingActionButton {
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
		let dialog_open_cb = self
			.props
			.parent_link
			.callback(|_: MouseEvent| crate::components::club_view::Msg::ShowDialog);

		html! {
			<div id="fab" class="club-fab">
				<button onclick=dialog_open_cb>
					{ "+" }
				</button>
			</div>
		}
	}
}
