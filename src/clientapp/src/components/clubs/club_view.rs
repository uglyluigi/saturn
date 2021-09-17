use anyhow::*;
use serde::{Deserialize, Serialize};
use yew::{
	format::{Json, Nothing},
	prelude::*,
	services::fetch::{FetchService, FetchTask, Request, Response, StatusCode},
	utils::document,
	virtual_dom::VNode,
	web_sys::{Element, Node},
	Html,
	Properties,
};

use crate::{
	components::{
		clubs::CreateClubFloatingActionButton as Fab,
		core::{router::*, Toolbar},
		ClubCard,
		ClubDialog,
	},
	tell,
	types::*,
};



pub struct ClubView {
	link: ComponentLink<Self>,
	props: Props,
	redirect: Redirect,

	show_dialog: bool,

	// Collections
	fetch_tasks: Vec<FetchTask>,

	clubs_fetch_state: FetchState<Vec<ClubDetails>>,
	auth_details_fetch_state: FetchState<AuthDetails>,
	user_details_fetch_state: FetchState<UserDetails>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
	pub first_name: String,
	pub last_name: String,
}

pub enum Msg {
	// Gets
	GetUserDetails,
	GetAuthDetails,
	GetClubDetails(Option<i32>),

	// Receives
	ReceiveUserDetails(Option<UserDetails>),
	ReceiveAuthDetails(Option<AuthDetails>),
	ReceiveClubDetails(Option<Vec<ClubDetails>>),

	// Other
	RequestLogin,
	ShowDialog,
	HideDialog,
}

enum Redirect {
	No,
	Yes(AppRoute),
}

use Redirect::*;

impl ClubView {
	pub fn push_task(&mut self, task: FetchTask) {
		self.fetch_tasks.push(task);
	}

	pub fn clean_tasks(&mut self) {
		use yew::services::Task;
		let mut index: Option<usize> = None;

		for (i, e) in self.fetch_tasks.iter().enumerate() {
			if !e.is_active() {
				index = Some(i);
				break;
			}
		}

		if let Some(i) = index {
			drop(self.fetch_tasks.remove(i));
			self.clean_tasks();
		}
	}

	fn generate_club_list(&self) -> Html {
		if let FetchState::Done(clubs) = &self.clubs_fetch_state {
			if clubs.len() > 0 {
				html! {
					{
						for clubs.iter().map(|x| {
							html! {
								<ClubCard vote_count=x.member_count.clone() as i32 club_name=x.name.clone() club_description=x.name.clone() organizer_name=String::from("TODO")/>
							}
						})
					}
				}
			} else {
				html! {
					<h2>{ "Be the first to make a club!" }</h2>
				}
			}
		} else {
			html! {
				<h2>{"Failed to load clubs!"}</h2>
			}
		}
	}
}

impl Component for ClubView {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			props,
			fetch_tasks: vec![],
			redirect: No,
			show_dialog: false,
			clubs_fetch_state: FetchState::Waiting,
			auth_details_fetch_state: FetchState::Waiting,
			user_details_fetch_state: FetchState::Waiting,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		use Msg::*;

		self.clean_tasks();

		match msg {
			GetUserDetails => {
				let req = yew::services::fetch::Request::get("/api/user/details")
					.body(yew::format::Nothing);

				self.user_details_fetch_state = FetchState::Waiting;

				match req {
					Ok(req) => {
						let callback = self.link.callback(
							|response: Response<Json<Result<UserDetails, anyhow::Error>>>| {
								match response.status() {
									StatusCode::OK => {
										tell!("Got user details");
										let Json(body) = response.into_body();

										match body {
											Ok(deets) => ReceiveUserDetails(Some(deets)),
											Err(err) => {
												tell!("Failed to deser user data: {}", err);
												Msg::ReceiveUserDetails(None)
											}
										}
									}

									StatusCode::FORBIDDEN => Msg::RequestLogin,

									_ => {
										tell!(
											"Failed to receive user data: status code {}",
											response.status()
										);
										Msg::ReceiveUserDetails(None)
									}
								}
							},
						);

						match yew::services::fetch::FetchService::fetch(req, callback) {
							Ok(task) => {
								self.push_task(task);
							}
							Err(err) => {}
						}
					}

					Err(err) => {
						tell!("Failed to build request for user details: {:?}", err);
					}
				}
			}

			GetAuthDetails => {
				let req = yew::services::fetch::Request::get("/api/auth/details")
					.body(yew::format::Nothing);

				match req {
					Ok(req) => {
						let callback = self.link.callback(
							|response: Response<Json<Result<UserDetails, anyhow::Error>>>| {
								match response.status() {
									StatusCode::OK => {
										tell!("Got auth details");
										let Json(body) = response.into_body();

										match body {
											Ok(deets) => ReceiveUserDetails(Some(deets)),
											Err(err) => {
												tell!("Failed to deser auth data: {}", err);
												Msg::ReceiveUserDetails(None)
											}
										}
									}

									StatusCode::FORBIDDEN => Msg::RequestLogin,

									_ => {
										tell!(
											"Failed to receive auth data: status code {}",
											response.status()
										);
										Msg::ReceiveUserDetails(None)
									}
								}
							},
						);

						match yew::services::fetch::FetchService::fetch(req, callback) {
							Ok(task) => {
								self.push_task(task);
							}
							Err(err) => {}
						}
					}

					Err(err) => {
						tell!("Failed to build request for auth details: {:?}", err);
					}
				}
			}

			GetClubDetails(id) => {
				let req =
					yew::services::fetch::Request::get("/api/clubs").body(yew::format::Nothing);

				match req {
					Ok(req) => {
						// TODO the response type in this callback is probably gonna have to change when all clubs are gotten from backend
						let callback = self.link.callback(
							|response: Response<Json<Result<Vec<ClubDetails>, anyhow::Error>>>| {
								match response.status() {
									StatusCode::OK => {
										tell!("Got club details");
										let Json(body) = response.into_body();

										match body {
											Ok(deets) => ReceiveClubDetails(Some(deets)),
											Err(err) => {
												tell!("Failed to deser auth data: {}", err);
												Msg::ReceiveClubDetails(None)
											}
										}
									}

									StatusCode::FORBIDDEN => Msg::RequestLogin,

									_ => {
										tell!(
											"Failed to receive auth data: status code {}",
											response.status()
										);
										Msg::ReceiveClubDetails(None)
									}
								}
							},
						);

						match yew::services::fetch::FetchService::fetch(req, callback) {
							Ok(task) => {
								self.push_task(task);
							}
							Err(err) => {}
						}
					}

					Err(err) => {
						tell!("Failed to build request for auth details: {:?}", err);
					}
				}
			}

			ReceiveUserDetails(deets) => {
				self.user_details_fetch_state = if let Some(deet) = deets {
					FetchState::Done(deet)
				} else {
					FetchState::Failed(Some(anyhow!("Failed to get user details (struct was none)")))
				}
			}

			ReceiveAuthDetails(deets) => {
				self.auth_details_fetch_state = if let Some(deet) = deets {
					FetchState::Done(deet)
				} else {
					FetchState::Failed(Some(anyhow!("Failed to get auth details (struct was none)")))
				}
			}

			ReceiveClubDetails(deets) => {
				self.clubs_fetch_state = if let Some(deet) = deets {
					FetchState::Done(deet)
				} else {
					FetchState::Failed(Some(anyhow!("Failed to get club details (struct was none)")))
				}
			}

			RequestLogin => {
				// I like how this really looks like a stupid pseudocode example.
				// I need coffee haha = Laughter(Kind::Insincere)
				self.redirect = Yes(AppRoute::Login);
			}

			ShowDialog => {
				self.show_dialog = true;
			}

			HideDialog => {
				self.show_dialog = false;
			}
		}

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		if let Yes(route) = &self.redirect {
			html! {
				<AppRedirect route=route.clone()/>
			}
		} else {
			let fab_open_cb = self.link.callback(|_: MouseEvent| Msg::ShowDialog);

			html! {
				<div class="club-view">
					<Toolbar username=self.props.first_name.clone()/>
					<ClubCard vote_count=0 organizer_name=String::from("Sans Undertale") club_name=String::from("Southeastern Undertale Club") club_description=String::from("The coolest club ever")/>

					{ self.generate_club_list() }

					<Fab parent_link=self.link.clone()/>
					<ClubDialog show=self.show_dialog parent_link=self.link.clone()/>
				</div>
			}
		}
	}

	fn rendered(&mut self, first: bool) {
		if first {
			self.link.send_message(Msg::GetClubDetails(None));
		}
	}
}
