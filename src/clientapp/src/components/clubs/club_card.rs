

use gloo_dialogs::confirm;

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlElement};
use yew::{
	prelude::*,
	services::fetch::{FetchService, FetchTask, Request, Response, StatusCode},
	Properties,
};

use crate::{
	components::{clubs::pg_details, core::router::*, ClubView},
	event::{AgentMessage, Amogus},
	tell,
	types::*,
};

// The component representing the cards that live inside the club view.
pub struct ClubCard {
	link: ComponentLink<Self>,
	props: Props,

	delete_fetch_state: Option<FetchState<()>>,
	delete_fetch_task: Option<FetchTask>,

	join_fetch_state: Option<FetchState<()>>,
	join_fetch_task: Option<FetchTask>,

	leave_fetch_state: Option<FetchState<()>>,
	leave_fetch_task: Option<FetchTask>,

	number_ref: NodeRef,
	body_ref: NodeRef,
	member_count: i64,

	number_spin_anim_end_cb: Option<Closure<dyn Fn()>>,

	which_button: JoinButton,

	show_details: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub details: Mlk<ClubDetails>,
	// This link is used to send messages to the ClubView, like when a club card is deleted.
	pub parent_link: Mlk<ComponentLink<ClubView>>,
	#[prop_or(0.0)]
	// This is the delay that the card should wait for before playing its animation. Used to stagger the fade/drop-in animation in the club view.
	pub reveal_delay: f32,
}

pub enum Msg {
	// Sent when a card is deleted and confirmed.
	Delet,
	// Sent when the club's record on the backend has been successfully removed.
	DoneDelet,

	// Sent when the star button is pressed. Send a request to the backend to add
	// whatever user is logged in to the list of club members.
	Join,
	// Sent when the backend responds OK and the club is joined.
	DoneJoin,

	// Sent when the star button is pressed. Send a request to the backend to remove
	// the logged-in user from the member list for that particular club.
	Leave,
	// Sent when the backend has successfully removed the user from the club's member list.
	DoneLeave,

	// Sent when the spin animation that plays on the count completes.
	AnimDone,
	ShowDetails,

	SendDetails,
}

// An enum used to represent the current state of the star button,
// whether it should be displayed filled or not.
#[derive(Copy, Clone)]
enum JoinButton {
	EmptyStar,
	FilledStar,
}

// This lets me just put a JoinButton varient inside of the html! macro without having
// to perform any sort of conversion or matching. It just implicitly into()s the variant
// into its VNode representation.
impl std::convert::Into<Html> for JoinButton {
	fn into(self) -> Html {
		match self {
			JoinButton::EmptyStar => html! {
				<span class="material-icons">{"star_outline"}</span>
			},
			JoinButton::FilledStar => html! {
				<span class="material-icons">{"star"}</span>
			},
		}
	}
}

impl ClubCard {
	// A function that is called in the view function. Either returns a VNode containing
	// a delete button or an empty VNode, depending on whether or not the user is
	// authorized to delete the club or not.
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
		

		let _colors = vec!["ED6A5A", "FF5964", "2BC29F"];

		Self {
			link,
			which_button: if props.details.unwrap().is_member {
				JoinButton::FilledStar
			} else {
				JoinButton::EmptyStar
			},

			delete_fetch_state: None,
			delete_fetch_task: None,

			join_fetch_state: None,
			join_fetch_task: None,

			leave_fetch_state: None,
			leave_fetch_task: None,

			number_ref: NodeRef::default(),
			body_ref: NodeRef::default(),

			member_count: props.details.unwrap().member_count.clone(),

			props,

			number_spin_anim_end_cb: None,
			show_details: false,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			// Deletes the club represented by this card.
			Msg::Delet => {
				// opens confirm dialog
				let result = confirm("Are you sure you want to delete?");
				if result {
					let req =
						Request::delete(format!("/api/clubs/{}", self.props.details.unwrap().id))
							.body(yew::format::Nothing);

					//FIXME back end tends to return 408 for delete requests which does actually delete it but sometimes it does so too slowly
					//to be reflected on the front end the first time it is refreshed.
					match req {
						Ok(req) => {
							let callback = self.link.callback(
								|response: Response<Result<String, anyhow::Error>>| {
									match response.status() {
										StatusCode::OK => {
											tell!("Successfully deleted club");
										}

										_ => {
											tell!("Weird status received: {}", response.status());
										}
									};

									Msg::DoneDelet
								},
							);

							match FetchService::fetch(req, callback) {
								Ok(task) => {
									self.delete_fetch_task = Some(task);
									self.delete_fetch_state = Some(FetchState::Waiting);
								}
								Err(e) => {
									self.delete_fetch_state =
										Some(FetchState::Failed(Some(anyhow::anyhow!(e))));
								}
							}
						}
						Err(_err) => (),
					}

					self.props
						.parent_link
						.unwrap()
						.send_message(crate::components::club_view::Msg::GetClubDetails(None));
				}
			}

			// Sent when the fetch service finishes deleting the club.
			Msg::DoneDelet => {
				self.delete_fetch_state = Some(FetchState::Done(()));
				drop(self.delete_fetch_task.take());

				// Play delete animation
				let el = self.body_ref.cast::<HtmlElement>().unwrap();
				el.style().remove_property("animation").unwrap();

				let link_clone = self.props.parent_link.unwrap().clone();
				el.set_onanimationend(Some(
					Closure::once_into_js(move || {
						link_clone
							.send_message(crate::components::club_view::Msg::GetClubDetails(None))
					})
					.unchecked_ref(),
				));
				el.class_list().add_1("disappear").unwrap();
			}

			// Sent when the button is pressed by a user that has not previously joined this club.
			Msg::Join => {
				let req = Request::put(format!(
					"/api/clubs/{}/join",
					self.props.details.unwrap().id
				))
				.body(yew::format::Nothing);

				match req {
					Ok(req) => {
						let callback = self.link.callback(
							|response: Response<Result<String, anyhow::Error>>| {
								match response.status() {
									StatusCode::OK => {
										tell!("Successfully joined club");
									}

									_ => {
										tell!("Weird status received: {}", response.status());
									}
								};

								Msg::DoneJoin
							},
						);

						match FetchService::fetch(req, callback) {
							Ok(task) => {
								self.join_fetch_task = Some(task);
								self.join_fetch_state = Some(FetchState::Waiting);
							}
							Err(e) => {
								self.join_fetch_state =
									Some(FetchState::Failed(Some(anyhow::anyhow!(e))));
							}
						}
					}
					Err(_err) => (),
				}
			}

			// Sent when the fetch service has added the logged in user to the member list.
			Msg::DoneJoin => {
				// Get the element and activate the transition
				let el = self.number_ref.cast::<HtmlElement>().unwrap();
				el.class_list().add_1("number-spin-in").unwrap();
				self.which_button = JoinButton::FilledStar;
				drop(self.delete_fetch_task.take());
			}

			// Sent when the user tries to leave a club.
			Msg::Leave => {
				let req = Request::put(format!(
					"/api/clubs/{}/leave",
					self.props.details.unwrap().id
				))
				.body(yew::format::Nothing);

				match req {
					Ok(req) => {
						let callback = self.link.callback(
							|response: Response<Result<String, anyhow::Error>>| {
								match response.status() {
									StatusCode::OK => {
										tell!("Successfully left club");
									}

									_ => {
										tell!("Weird status received: {}", response.status());
									}
								};

								Msg::DoneLeave
							},
						);

						match FetchService::fetch(req, callback) {
							Ok(task) => {
								self.leave_fetch_task = Some(task);
								self.leave_fetch_state = Some(FetchState::Waiting);
							}
							Err(e) => {
								self.leave_fetch_state =
									Some(FetchState::Failed(Some(anyhow::anyhow!(e))));
							}
						}
					}
					Err(_err) => (),
				}
			}

			Msg::DoneLeave => {
				// Get element, activate transition
				let el = self.number_ref.cast::<web_sys::HtmlElement>().unwrap();
				el.class_list().add_1("number-spin-out").unwrap();
				self.which_button = JoinButton::EmptyStar;
				drop(self.leave_fetch_task.take());
			}

			Msg::AnimDone => {
				let el = self.number_ref.cast::<HtmlElement>().unwrap();
				let classes = el.class_list();

				if classes.contains("number-spin-in") {
					classes.remove_1("number-spin-in").unwrap();
					self.member_count += 1;
				} else if classes.contains("number-spin-out") {
					classes.remove_1("number-spin-out").unwrap();
					self.member_count -= 1;
					//
				}
			}

			Msg::ShowDetails => {
				self.show_details = true;
				self.link.send_message(Msg::SendDetails);
			}

			Msg::SendDetails => {
				// Send the details to the details page via the
				let mut dispatcher = Amogus::dispatcher();
				dispatcher.send(crate::event::Request::EventBusMsg(
					AgentMessage::DetailsPageMsg(pg_details::Msg::AcceptDetails(
						self.props.details.clone().unwrap_into(),
					)),
				));
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

		let open_details = self.link.callback(move |_: MouseEvent| Msg::ShowDetails);

		html! {
			<div>
				{
					if self.show_details {
						html! {
							<>
								<AppRedirect route=AppRoute::Details {id: self.props.details.unwrap().id as usize}/>
							</>
						}
					} else {
						html! {
							<>
							</>
						}
					}
				}
				<div ref={self.body_ref.clone()} class="club-card">
					<div class="club-card-header">
						<h1>{self.props.details.unwrap().name.clone()}</h1>
					</div>

					<hr/>


					<div class="club-card-body">
						<div id="left-col">
							<img src={format!("/assets/clubs/{}.png", self.props.details.unwrap().id)}/>
						</div>

						<div id="right-col">
							<p>{"Organizer"}</p>
							<h3>{format!("{} {}", self.props.details.unwrap().head_moderator.first_name, self.props.details.unwrap().head_moderator.last_name)}</h3>

							<hr/>
							<h3>
								<div id="interested">
									<div ref=self.number_ref.clone() id="member-number">
										{self.member_count}
									</div>
									<div>
										{"interested"}
									</div>
								</div>
							</h3>

							<hr/>

							<div class="club-card-action-bar">
								<button id="club-card-join-btn" onclick={match self.which_button { JoinButton::FilledStar => leave_club, _ => join_club }}>
									<abbr data_title={match self.which_button { JoinButton::FilledStar => "Not Interested", _ => "Interested"}}>
										{self.which_button}
									</abbr>
								</button>
								<button onclick=open_details id="club-card-expand-btn"><abbr data_title="Details"><span class="material-icons">{"open_in_full"}</span></abbr></button>

								{
									if self.props.details.unwrap().is_moderator != "false" {
										html! {
											<button id="club-card-delete-btn" onclick=delete_club><abbr data_title="Delete"><span class="material-icons">{"close"}</span></abbr></button>
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
					</div>
				</div>
			</div>
		}
	}

	fn rendered(&mut self, first: bool) {
		// A bunch of complicated nonsense related to the number animation.
		if first {
			let el = self.number_ref.cast::<HtmlElement>().unwrap();
			let link_clone = self.link.clone();

			let cb = Closure::wrap(Box::new(move || {
				link_clone.send_message(Msg::AnimDone);
			}) as Box<dyn Fn()>);

			el.set_ontransitionend(Some(cb.as_ref().unchecked_ref()));
			self.number_spin_anim_end_cb = Some(cb);

			let body = self.body_ref.cast::<HtmlElement>().unwrap();
			body.style()
				.set_property(
					"animation",
					format!(
						"reveal-cards 0.3s linear {}s forwards",
						self.props.reveal_delay
					)
					.as_str(),
				)
				.unwrap();
		}
	}
}
