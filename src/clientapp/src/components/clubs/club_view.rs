use anyhow::*;
use serde::{Deserialize, Serialize};
use web_sys::HtmlElement;
use yew::{Html, Properties, format::{Json, Nothing}, prelude::*, services::fetch::{FetchService, FetchTask, Request, Response, StatusCode}, utils::document, virtual_dom::{VComp, VNode}, web_sys::{Element, Node}};
use js_sys::Function;

use crate::{
	components::{
		clubs::CreateClubFloatingActionButton as Fab,
		coolshit::Spinner,
		core::{router::*, Toolbar},
		ClubCard,
	},
	tell,
	types::*,
	flags::*,
};

pub struct ClubView {
	link: ComponentLink<Self>,
	props: Props,
	redirect: Redirect,

	show_dialog: bool,
	dialog_display_class: Option<String>,

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
	FakeGettingClubs,
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

	pub fn make_cards(&self, vec: &Vec<ClubDetails>) -> Html {
		let mut i = 0.1;

		html! {
			<>
				{
					for vec.iter().map(|x| {
						tell!("Makin cards");

						html! {
							<ClubCard
								details=Mlk::new(x.clone())
								parent_link=Mlk::new(self.link.clone())
								reveal_delay={i += 0.1; i}
							/>
						}
					})
				}
			</>
		}
	}

	pub fn normal_view(&self) -> Html {
		match &self.clubs_fetch_state {
			FetchState::Done(clubs) => {
				if clubs.len() > 0 {
					html! {
						<>
							{
								self.make_cards(clubs)
							}
						</>
					}
				} else {
					html! {
						<></>
					}
				}
			},

			_ => html! { <> </> }
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
			dialog_display_class: None,
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
				self.clubs_fetch_state = FetchState::Waiting;

				let req =
					yew::services::fetch::Request::get("/api/clubs").body(yew::format::Nothing);

				match req {
					Ok(req) => {
						let callback = self.link.callback(
							|response: Response<Json<Result<Vec<ClubDetails>, anyhow::Error>>>| {
								match response.status() {
									StatusCode::OK => {
										tell!("Got club details");
										let Json(body) = response.into_body();

										match body {
											Ok(deets) => {
												tell!("Received deets: {:?}", deets);
												ReceiveClubDetails(Some(deets))
											},
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
					FetchState::Failed(Some(anyhow!(
						"Failed to get user details (struct was none)"
					)))
				}
			}

			ReceiveAuthDetails(deets) => {
				self.auth_details_fetch_state = if let Some(deet) = deets {
					FetchState::Done(deet)
				} else {
					FetchState::Failed(Some(anyhow!(
						"Failed to get auth details (struct was none)"
					)))
				}
			}

			ReceiveClubDetails(deets) => {
				self.clubs_fetch_state = if let Some(deet) = deets {
					FetchState::Done(deet)
				} else {
					FetchState::Failed(Some(anyhow!(
						"Failed to get club details (struct was none)"
					)))
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
			},

			FakeGettingClubs => {
				self.clubs_fetch_state = FetchState::Done(vec![]);
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
			html! {
				<>				
					<div class="content club-view-fetch-info">
						{
							
							match &self.clubs_fetch_state {
								FetchState::Failed(maybe_msg) => {
									html! {
										<span class="bad">
											<h2>{"Failed to fetch clubs."}</h2>
										</span>
									}
								},
					
								FetchState::Waiting => {
									html! {
										<span class="fetching">
											<h2>{"Fetching clubs"}</h2>
											<Spinner/>
										</span>
									}
								},

								FetchState::Done(deets) => {
									if deets.len() == 0 {
										html! {
											<span class="bad">
												<h2>
													{ "Be the first to post your own club!" }
												</h2>
											</span>
										}
									} else {
										html! {
											<>
											</>
										}
									}
								}
							}
						}
					</div>	

					<div class="club-view">
						{
							self.normal_view()
						}
					</div>
				</>
			}
		}
	}

	fn rendered(&mut self, first: bool) {
		if first {
			self.link.send_message(Msg::GetClubDetails(None));
		}
	}
}
