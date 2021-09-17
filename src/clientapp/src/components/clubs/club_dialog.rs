use js_sys::Object;
use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlElement};
use yew::services::fetch::FetchTask;
use yew::{prelude::*, Html, ShouldRender};

use crate::components::ClubView;
use crate::tell;
use crate::types::FetchState;

pub struct ClubDialog {
	link: ComponentLink<Self>,
	club_name_field_contents: Option<String>,
	club_body_field_contents: Option<String>,
	props: Props,
	post_task: Option<FetchTask>,
	post_task_state: FetchState<()>,
}

#[derive(Properties, Debug, Clone)]
pub struct Props {
	pub parent_link: ComponentLink<ClubView>,
	pub show: bool,
}

pub enum Msg {
	Open,
	Close,
	Ignore,
	UpdateInfoState(WhichTextField, String),
	ValidateForm,
	PostClub
}

pub enum WhichTextField {
	TheNameOne,
	TheBodyOne
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
			post_task: None,
			post_task_state: FetchState::Waiting
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Open => (),
			Msg::Close => (),
			Msg::Ignore => (),
			Msg::UpdateInfoState(which, value) => {
				match which {
					WhichTextField::TheBodyOne => {
						self.club_body_field_contents = if value.len() > 0 {
							Some(value)
						} else {
							None
						}
					},

					WhichTextField::TheNameOne => {
						self.club_name_field_contents = if value.len() > 0 {
							Some(value)
						} else {
							None
						}
					}
				}
			},

			Msg::ValidateForm => {
				self.link.send_message(Msg::PostClub);
			},

			Msg::PostClub => {
				
			}
		}

		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.props = props;
		true
	}

	fn view(&self) -> Html {
		let close_cb = self
			.props
			.parent_link
			.callback(move |_: MouseEvent| crate::components::club_view::Msg::HideDialog);

		let club_name_field_callback = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateInfoState(WhichTextField::TheNameOne, data.value)
		});

		let club_body_field_callback = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateInfoState(WhichTextField::TheBodyOne, data.value)
		});

		html! {
			<div>
				<div style={if self.props.show.clone() {"visibility: visible;"} else {"visibility: hidden;"}} id="modal-bg">
					<div id="new-club-dialog">
						<div id="dialog-header">
							<h3>{"Create new club"}</h3>
							<button id="close-x-button" onclick=close_cb.clone()>{"X"}</button>
						</div>

						<div id="dialog-content">
							<input type="text" oninput=club_name_field_callback placeholder="Club Name"/>
							<input type="text" oninput=club_body_field_callback placeholder="Club Body"/>
						</div>

						<div id="dialog-buttons">
							<button class="dialog-button" onclick=close_cb>{"Close"}</button>
							<button class="dialog-button" onclick=self.link.callback(|_: MouseEvent| Msg::ValidateForm)>{"OK"}</button>
						</div>
					</div>
				</div>
			</div>
		}
	}
}
