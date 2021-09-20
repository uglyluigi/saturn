use yew::format::Json;
use yew::services::FetchService;
use yew::services::fetch::{FetchTask, Request, Response, StatusCode};
use yew::{prelude::*, Html, ShouldRender};
use serde_json::json;
use serde_json::Value;
use anyhow::anyhow;

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
	pub dialog_anim_class: String,
	pub bg_anim_class: String,
}

pub enum Msg {
	Open,
	Close,
	Ignore,
	UpdateInfoState(WhichTextField, String),
	ValidateForm,

	PostClub,
	PostClubDone,
}

pub enum WhichTextField {
	TheNameOne,
	TheBodyOne
}

impl ClubDialog {
	fn close(&self) {
		let close_cb = self
			.props
			.parent_link
			.callback(move |_: Option<()>| crate::components::club_view::Msg::HideDialog);

		close_cb.emit(None);
	}

	fn reset(&mut self) {
		self.club_body_field_contents = None;
		self.club_name_field_contents = None;
		self.post_task = None;
		self.post_task_state = FetchState::Waiting;
	}
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
			Msg::Close => {
				self.props.dialog_anim_class = String::from("new-club-dialog-anim-out");
				self.props.bg_anim_class = String::from("modal-bg-anim-out");
				self.reset();
			},
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
				// TODO
				self.link.send_message(Msg::PostClub);
			},

			Msg::PostClub => {
				self.post_task_state = FetchState::Waiting;
				
				if let (Some(name), Some(body)) = (self.club_name_field_contents.clone(), self.club_body_field_contents.clone()) {
					let json = json!({"name": Value::String(name), "body": Value::String(body)});
					let request = Request::post("/api/clubs/create").body(Json(&json)).unwrap();

					let response_callback = self.link.callback(|response: Response<Json<Result<(), anyhow::Error>>>| {
						match response.status() {
							StatusCode::OK => {
								tell!("Successfully posted club");
								Msg::PostClubDone
							},
	
							_ => {
								tell!("Bad status receieved: {:?}", response.status());
								//Error stuff
								Msg::Ignore
							}
						}
					});

					match FetchService::fetch(request, response_callback) {
						Ok(task) => self.post_task = Some(task),
						Err(err) => {
							tell!("Failed to post club: {}", err);
							self.post_task_state = FetchState::Failed(Some(anyhow!(format!("{:?}", err))));
						}
					}
				}
			},

			Msg::PostClubDone => {
				tell!("Post club done!");
				self.close();
				self.reset();
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
			.link
			.callback( |_: MouseEvent| {
				Msg::Close
			});

		let club_name_field_callback = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateInfoState(WhichTextField::TheNameOne, data.value)
		});

		let club_body_field_callback = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateInfoState(WhichTextField::TheBodyOne, data.value)
		});

		html! {
			<div>
				<div class={self.props.bg_anim_class.clone()} id="modal-bg">
					<div class={self.props.dialog_anim_class.clone()} id="new-club-dialog">
						<div id="dialog-header">
							<h3>{"Create new club"}</h3>
							<button id="close-x-button" onclick=close_cb.clone()>{"X"}</button>
						</div>

						<div id="dialog-content">
							<input type="text" oninput=club_name_field_callback value=self.club_name_field_contents.clone() placeholder="Club Name"/>
							<input type="text" oninput=club_body_field_callback value=self.club_body_field_contents.clone() placeholder="Club Body"/>
						</div>

						<div id="dialog-buttons">
							<button class="dialog-button" id="club-dialog-close-btn" onclick=close_cb>{"Close"}</button>
							<button class="dialog-button" id="club-dialog-ok-btn" onclick=self.link.callback(|_: MouseEvent| Msg::ValidateForm)>{"OK"}</button>
						</div>
					</div>
				</div>
			</div>
		}
	}
}
