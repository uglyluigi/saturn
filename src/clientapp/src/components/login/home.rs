use anyhow::*;
use yew::{
	format::{Json, Nothing},
	prelude::*,
	services::fetch::{FetchService, FetchTask, Request, Response, StatusCode},
};

use crate::{
	components::{core::*, ClubView},
	tell,
	types::*,
	flags::*,
};

pub enum Msg {
	FetchUserInfo,
	ReceieveUserInfo(AuthDetails),
	FailToReceiveUserInfo(Option<anyhow::Error>),
}

pub struct Home {
	link: ComponentLink<Self>,
	fetch_task: Option<FetchTask>,
	fetch_state: FetchState<AuthDetails>,
	details: Option<AuthDetails>,
}

impl Component for Home {
	type Message = Msg;
	type Properties = ();

	fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			fetch_task: None,
			details: None,
			fetch_state: FetchState::Waiting,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::FetchUserInfo => {
				self.fetch_state = FetchState::Waiting;
				let req = Request::get("/api/auth/details").body(Nothing);

				match req {
					Ok(req) => {
						let callback = self.link.callback(
							|response: Response<Json<Result<AuthDetails, anyhow::Error>>>| {
								match response.status() {
									StatusCode::OK => {
										tell!("OK response");
										let Json(body) = response.into_body();

										match body {
											Ok(deets) => match deets.auth_level {
												AuthLevel::Guest => Msg::FailToReceiveUserInfo(
													Some(anyhow!("Guests must log in")),
												),
												_ => Msg::ReceieveUserInfo(deets),
											},
											Err(err) => Msg::FailToReceiveUserInfo(Some(err)),
										}
									}

									_ => Msg::FailToReceiveUserInfo(Some(anyhow!(
										"Weird status code received: {}",
										response.status()
									))),
								}
							},
						);

						match FetchService::fetch(req, callback) {
							Ok(task) => self.fetch_task = Some(task),
							Err(err) => tell!("{}", err),
						}
					}

					Err(err) => tell!("Error building request: {}", err),
				}
			}

			Msg::ReceieveUserInfo(data) => {
				self.fetch_state = FetchState::Done(data);
				self.fetch_task = None;
			}

			Msg::FailToReceiveUserInfo(maybe_error) => {
				self.fetch_task = None;

				if let Some(error) = maybe_error {
					tell!("Error getting user details: {:?}", error);
					self.fetch_state = FetchState::Failed(Some(error));
				} else {
					tell!("Unspecified error getting user details");
					self.fetch_state = FetchState::Failed(None);
				}
			}
		}

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		if *IS_DEBUG_MODE {
			self.debug_view()
		} else {
			self.normal_view()
		}
	}
}

impl Home {
	fn debug_view(&self) -> Html {
		html! {
			<ClubView first_name=String::from("Adrian") last_name=String::from("Brody")/>
		}
	}

	fn normal_view(&self) -> Html {
		match &self.fetch_state {
			FetchState::Waiting => html! {
				<h1> {"Waiting..."} </h1>
			},

			FetchState::Done(details) => html! {
				//TODO handle token timeout. Just send Msg::RequestUserData again
				<ClubView first_name=details.first_name.clone().unwrap() last_name=details.last_name.clone().unwrap()/>
			},

			FetchState::Failed(maybe_error) => {
				match maybe_error {
					Some(err) => tell!("Error: {:?}", err),
					None => tell!("Unspecified error occurred."),
				};

				html! {
					<AppRedirect route=AppRoute::Login/>
				}
			}
		}
	}
}
