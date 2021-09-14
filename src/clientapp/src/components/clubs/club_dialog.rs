use yew::{prelude::*, Html, ShouldRender};

use crate::components::ClubView;

pub struct ClubDialog {
	link: ComponentLink<Self>,
	club_name_field_contents: Option<String>,
	club_body_field_contents: Option<String>,
	props: Props,
}

#[derive(Properties, Debug, Clone)]
pub struct Props {
	pub parent_link: ComponentLink<ClubView>,
	pub show: bool,
}

pub enum Msg {
	Open,
	Close,
}

impl Component for ClubDialog {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			club_name_field_contents: None,
			club_body_field_contents: None,
			props,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.props = props;
		true
	}

	fn view(&self) -> Html {
		let close_cb = self
			.props
			.parent_link
			.callback(move |e: MouseEvent| crate::components::club_view::Msg::HideDialog);

		html! {
			<div>
				<div style={if self.props.show.clone() {"visibility: visible;"} else {"visibility: hidden;"}} id="modal-bg">
					<div id="new-club-dialog">
						<div id="dialog-header">
							<h3>{"Create new club"}</h3>
							<button id="close-x-button" onclick=close_cb.clone()>{"X"}</button>
						</div>

						<div id="dialog-content">
							<input type="text" placeholder="Club Name"/>
							<input type="text" placeholder="Club Body"/>
						</div>

						<div id="dialog-buttons">
							<button class="dialog-button" onclick=close_cb>{"Close"}</button>
							<button class="dialog-button">{"OK"}</button>
						</div>
					</div>
				</div>
			</div>
		}
	}
}
