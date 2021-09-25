use yew::{prelude::*, Properties};
use crate::types::*;
use crate::components::ClubView;
use crate::tell;
use yew::services::fetch::{FetchService, FetchTask, Request, Response, StatusCode};


pub struct ClubCard {
	link: ComponentLink<Self>,
	props: Props,
	show_leave_button: bool,
	
	delete_fetch_state: Option<FetchState<()>>,
	delete_fetch_task: Option<FetchTask>,

	join_fetch_state: Option<FetchState<()>>,
	join_fetch_task: Option<FetchTask>,

	leave_fetch_state: Option<FetchState<()>>,
	leave_fetch_task: Option<FetchTask>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub details: Mlk<ClubDetails>,
	pub parent_link: Mlk<ComponentLink<ClubView>>,
	#[prop_or(String::new())]
	pub style: String,
}

pub enum Msg {
	ToggleLikeButton,

	Delet,
	DoneDelet,

	Join,
	DoneJoin,

	Leave,
	DoneLeave,
}

impl ClubCard {
	pub fn delete_btn(&self) -> Html {
		let delete = self.link.callback(|_: MouseEvent| Msg::Delet);

		if self.props.details.unwrap().is_moderator != "false" {
			html! {
				<button id="club-card-delete-btn" onclick=delete><span class="material-icons">{"close"}</span></button>
			}
		} else {
			html! {
				<></>
			}
		}
	}
}

impl Component for ClubCard {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		//TODO like button is already toggled based on if the user liked this club
		Self {
			link,
			props,
			show_leave_button: false,

			delete_fetch_state: None,
			delete_fetch_task: None,

			join_fetch_state: None,
			join_fetch_task: None,

			leave_fetch_state: None,
			leave_fetch_task: None
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::ToggleLikeButton => {

			},

			Msg::Delet => {
				let req = Request::delete(format!("/api/clubs/{}", self.props.details.unwrap().id)).body(yew::format::Nothing);

				match req {
					Ok(req) => {
						let callback = self.link.callback(|response: Response<Result<String, anyhow::Error>>| {
							match response.status() {
								StatusCode::OK => {
									tell!("Successfully deleted club");
								},

								_ => {
									tell!("Weird status received: {}", response.status());
								}
							};

							Msg::DoneDelet
						});

						match FetchService::fetch(req, callback) {
							Ok(task) => {
								self.delete_fetch_task = Some(task);
								self.delete_fetch_state = Some(FetchState::Waiting);
							},
							Err(e) => {
								self.delete_fetch_state = Some(FetchState::Failed(Some(anyhow::anyhow!(e))));
							},
						}
					},
					Err(err) => (),
				}

				self.props.parent_link.unwrap().send_message(crate::components::club_view::Msg::GetClubDetails(None));
			},

			Msg::DoneDelet => {
				self.delete_fetch_state = Some(FetchState::Done(()));
				self.delete_fetch_task = None;
				self.props.parent_link.unwrap().send_message(crate::components::club_view::Msg::GetClubDetails(None));
			},

			Msg::Join => {
				let req = Request::put(format!("/api/clubs/{}/join", self.props.details.unwrap().id)).body(yew::format::Nothing);

				match req {
					Ok(req) => {
						let callback = self.link.callback(|response: Response<Result<String, anyhow::Error>>| {
							match response.status() {
								StatusCode::OK => {
									tell!("Successfully joined club");
								},

								_ => {
									tell!("Weird status received: {}", response.status());
								}
							};

							Msg::DoneJoin
						});

						match FetchService::fetch(req, callback) {
							Ok(task) => {
								self.join_fetch_task = Some(task);
								self.join_fetch_state = Some(FetchState::Waiting);
							},
							Err(e) => {
								self.join_fetch_state = Some(FetchState::Failed(Some(anyhow::anyhow!(e))));
							},
						}
					},
					Err(err) => (),
				}
			},

			Msg::DoneJoin => {
				self.show_leave_button = true;
			},

			Msg::Leave => {
				let req = Request::put(format!("/api/{}/leave", self.props.details.unwrap().id)).body(yew::format::Nothing);

				match req {
					Ok(req) => {
						let callback = self.link.callback(|response: Response<Result<String, anyhow::Error>>| {
							match response.status() {
								StatusCode::OK => {
									tell!("Successfully left club");
								},

								_ => {
									tell!("Weird status received: {}", response.status());
								}
							};

							Msg::DoneLeave
						});

						match FetchService::fetch(req, callback) {
							Ok(task) => {
								self.leave_fetch_task = Some(task);
								self.leave_fetch_state = Some(FetchState::Waiting);
							},
							Err(e) => {
								self.leave_fetch_state = Some(FetchState::Failed(Some(anyhow::anyhow!(e))));
							},
						}
					},
					Err(err) => (),
				}
			},

			Msg::DoneLeave => {
				self.show_leave_button = false;
			}
		}

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		let delete_club = self.link.callback(|_: MouseEvent| Msg::Delet);
		let join_club = self.link.callback(|_: MouseEvent| Msg::Join);
		let leave_club = self.link.callback(|_: MouseEvent| Msg::Leave);

		html! {
			<div style={self.props.style.clone()} class="club-card">
				<div class="club-card-header">
					<h1>{self.props.details.unwrap().name.clone()}</h1>
				</div>
				<hr/>
				<div class="club-card-body">
					<div id="left-col">
						<h2>{self.props.details.unwrap().member_count} {" members"}</h2>
					</div>

					<div id="right-col">
						<img src={self.props.details.unwrap().head_moderator.picture.clone()}/>
						<p>{"Organizer"}</p>
						<h2>{format!("{} {}", self.props.details.unwrap().head_moderator.first_name, self.props.details.unwrap().head_moderator.last_name)}</h2>
					</div>
				</div>

				<div class="club-card-action-bar">
					{
						if self.props.details.unwrap().is_member || self.show_leave_button {
							html! {
								<button id="club-card-join-btn" onclick=leave_club><span class="material-icons">{"logout"}</span></button>
							}
						} else {
							html! {
								<button id="club-card-join-btn" onclick=join_club><span class="material-icons">{"login"}</span></button>
							}
						}
					}

					{
						if self.props.details.unwrap().is_moderator != "false" || * crate::flags::IS_DEBUG_MODE {
							html! {
								<button id="club-card-delete-btn" onclick=delete_club><span class="material-icons">{"close"}</span></button>
							}
						} else {
							html! {
								<>
								</>
							}
						}
					}
				</div>
			</div>
		}
	}
}
