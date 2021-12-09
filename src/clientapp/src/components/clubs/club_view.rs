use std::cmp::Ordering;

use anyhow::*;


use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement};
use yew::{
	format::{Json, Nothing},
	prelude::*,
	services::fetch::{FetchService, FetchTask, Request, Response, StatusCode},
	utils::document,
	virtual_dom::{VComp, VNode},
	web_sys::{Element, Node},
	Html,
	Properties,
};

use crate::{
	components::{coolshit::Spinner, core::router::*, ClubCard},
	tell,
	types::*,
};

pub struct ClubView {
	link: ComponentLink<Self>,
	props: Props,
	redirect: Redirect,

	show_dialog: bool,
	dialog_display_class: Option<String>,

	// Collections
	fetch_tasks: Vec<FetchTask>,

	clubs_fetch_state: FetchState<()>,
	auth_details_fetch_state: FetchState<AuthDetails>,
	user_details_fetch_state: FetchState<UserDetails>,

	interested_radio_button_ref: NodeRef,
	most_popular_radio_button_ref: NodeRef,
	moderated_radio_button_ref: NodeRef,
	clubs: Vec<ClubDetails>,
	show_cards: bool,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
	#[prop_or(Mlk::new(None))]
	pub search_filter_function: Mlk<Option<fn(&String, &ClubDetails) -> bool>>,
	#[prop_or(None)]
	pub search_filter_text: Option<String>,
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
	Ignore,

	// Updates which rank to use. The thing wrapped in the enum is the button that was just selected
	UpdateRankState(HtmlButtonElement),
	UpdateClubSort,
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

	pub fn get_radio_buttons(&self) -> [HtmlButtonElement; 3] {
		let interested_button = self
			.interested_radio_button_ref
			.cast::<HtmlButtonElement>()
			.unwrap();
		let moderated_button = self
			.moderated_radio_button_ref
			.cast::<HtmlButtonElement>()
			.unwrap();
		let popular_button = self
			.most_popular_radio_button_ref
			.cast::<HtmlButtonElement>()
			.unwrap();

		[interested_button, moderated_button, popular_button]
	}

	pub fn make_cards(&self) -> Html {
		let mut i = 0.1;

		html! {
			<>
				{
					for self.clubs.iter().map(|x| {

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

	pub fn sort_clubs(&mut self) {
		let [interested_button, moderated_button, popular_button] = self.get_radio_buttons();

		// Unsurprisingly, these sort functions sort in ascending order. However, this is not super useful
		// when you want the items you consider to be "higher" to be towards the front of the list (which is
		// basically descending order). As such these are basically sorting backwards with "greater" items
		// at lower indices. That way, clubs that are meeting whatever qualifications set forth by
		// the ranker are towards the top of the page when displayed.

		if interested_button.class_list().contains("active-rank") {
			self.clubs.sort_by(|x, y| {
				if x.is_member == y.is_member {
					Ordering::Equal
				} else {
					if x.is_member && !y.is_member {
						Ordering::Less
					} else {
						Ordering::Greater
					}
				}
			});
		} else if moderated_button.class_list().contains("active-rank") {
			self.clubs.sort_by(|x, y| {
				if x.is_moderator == y.is_moderator {
					Ordering::Equal
				} else {
					if (x.is_moderator == "true" || x.is_moderator == "head")
						&& y.is_moderator == "false"
					{
						Ordering::Less
					} else {
						Ordering::Greater
					}
				}
			});
		} else if popular_button.class_list().contains("active-rank") {
			// This is pretty big brain if you ask me. Since it's sorting in ascending
			// order by member count, all you need to do is make the higher member counts
			// appear "lower" than the lower member counts. An incredibly simple way
			// to do this is to just negate the member count.
			self.clubs.sort_by_key(|x| -x.member_count);
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
			moderated_radio_button_ref: NodeRef::default(),
			interested_radio_button_ref: NodeRef::default(),
			most_popular_radio_button_ref: NodeRef::default(),
			clubs: vec![],
			show_cards: true,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		use Msg::*;

		self.clean_tasks();

		match msg {
			Ignore => (),

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
							Err(_err) => {}
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
							Err(_err) => {}
						}
					}

					Err(err) => {
						tell!("Failed to build request for auth details: {:?}", err);
					}
				}
			}

			GetClubDetails(_id) => {
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
							Err(_err) => {}
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
					let mut v = deet;

					if let Some(f) = self.props.search_filter_function.unwrap() {
						let search_text = self.props.search_filter_text.as_ref().expect("If you provide a search_filter_function to this component you must also provide search_filter_text.");
						v.retain(|e| f(&search_text, e))
					}

					self.clubs = v;

					self.interested_radio_button_ref
						.cast::<HtmlButtonElement>()
						.unwrap()
						.click();
					self.sort_clubs();

					FetchState::Done(())
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
			}

			FakeGettingClubs => {
				self.clubs_fetch_state = FetchState::Done(());
			}

			UpdateRankState(el) => {
				self.show_cards = false;
				for butt in self.get_radio_buttons() {
					if butt.class_list().contains("active-rank") {
						butt.class_list().remove_1("active-rank").unwrap();
					}
				}

				el.class_list().add_1("active-rank").unwrap();

				self.link.send_message(Msg::UpdateClubSort);
			}

			UpdateClubSort => {
				self.sort_clubs();
				self.show_cards = true;
			}
		}

		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.props = props;
		true
	}

	fn view(&self) -> Html {
		let on_clicc = self.link.callback(|e: MouseEvent| {
			let target = e.target().unwrap().dyn_into::<HtmlButtonElement>().unwrap();

			Msg::UpdateRankState(target)
		});

		if let Yes(route) = &self.redirect {
			html! {
				<AppRedirect route=route.clone()/>
			}
		} else {
			html! {
				<>
					<div class="club-filters">
						<div class="content">
							<h3><i>{"Rank by..."}</i></h3>


							<div class="ranks">
								<button onclick=on_clicc.clone() class="rank-button" ref=self.interested_radio_button_ref.clone()>
									<span class="material-icons">
										{"done"}
									</span>
									{"Interested"}
								</button>
								<button onclick=on_clicc.clone() class="rank-button" ref=self.most_popular_radio_button_ref.clone()>
									<span class="material-icons">
										{"done"}
									</span>
									{"Most popular"}
								</button>
								<button onclick=on_clicc.clone() class="rank-button" ref=self.moderated_radio_button_ref.clone()>
									<span class="material-icons">
										{"done"}
									</span>
									{"Moderated"}
								</button>
							</div>
						</div>
					</div>

					<div class="club-view-fetch-info">

					{

							match &self.clubs_fetch_state {
								FetchState::Failed(_maybe_msg) => {
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

								FetchState::Done(()) => {
									if self.clubs.len() == 0 {
										html! {
											<span class="bad">
												<h2>
													{
														if self.props.search_filter_function.unwrap().is_some() {
															"No matching clubs found."
														} else {
															"Be the first to post your own club!"
														}
													}
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

					<div>
						{
							match &self.clubs_fetch_state {
								FetchState::Done(()) => {
									if self.clubs.len() > 0 {
										html! {
											if self.show_cards {
												html! {
													<div class="club-view">
														{
															self.make_cards()
														}
													</div>
												}
											} else {
												html! {
													<>
													</>
												}
											}
										}
									} else {
										html! {
											<></>
										}
									}
								}

								_ => html! { <> </> },
							}
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
