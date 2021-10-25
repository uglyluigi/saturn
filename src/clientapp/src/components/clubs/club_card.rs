use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{AnimationEffect, HtmlElement};
use yew::{prelude::*, Properties};
use crate::types::*;
use crate::components::ClubView;
use crate::tell;
use yew::services::fetch::{FetchService, FetchTask, Request, Response, StatusCode};
use regex::{self, Regex};


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
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub details: Mlk<ClubDetails>,
	pub parent_link: Mlk<ComponentLink<ClubView>>,
	#[prop_or(0.0)]
	pub reveal_delay: f32,
}

pub enum Msg {
	ToggleLikeButton,

	Delet,
	DoneDelet,

	Join,
	DoneJoin,

	Leave,
	DoneLeave,

	AnimDone,
}

#[derive(Copy, Clone)]
enum JoinButton {
	EmptyStar,
	FilledStar
}

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
		Self {
			link,
			which_button: if props.details.unwrap().is_member { JoinButton::FilledStar } else { JoinButton::EmptyStar },

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
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::ToggleLikeButton => {

			},

			Msg::Delet => {
				let req = Request::delete(format!("/api/clubs/{}", self.props.details.unwrap().id)).body(yew::format::Nothing);

				//FIXME back end tends to return 408 for delete requests which does actually delete it but sometimes it does so too slowly
				//to be reflected on the front end the first time it is refreshed.
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
				drop(self.delete_fetch_task.take());

				// Play delete animation
				let el = self.body_ref.cast::<HtmlElement>().unwrap();
				el.style().remove_property("animation").unwrap();

				let link_clone = self.props.parent_link.unwrap().clone();
				el.set_onanimationend(Some(Closure::once_into_js(move || link_clone.send_message(crate::components::club_view::Msg::GetClubDetails(None))).unchecked_ref()));
				el.class_list().add_1("disappear").unwrap();
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
				// Get the element and activate the transition
				let el = self.number_ref.cast::<HtmlElement>().unwrap();
				el.class_list().add_1("number-spin-in").unwrap();
				self.which_button = JoinButton::FilledStar;
				drop(self.delete_fetch_task.take());
			},

			Msg::Leave => {
				let req = Request::put(format!("/api/clubs/{}/leave", self.props.details.unwrap().id)).body(yew::format::Nothing);

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
				// Get element, activate transition
				let el = self.number_ref.cast::<web_sys::HtmlElement>().unwrap();
				el.class_list().add_1("number-spin-out").unwrap();
				self.which_button = JoinButton::EmptyStar;
				drop(self.leave_fetch_task.take());
			},

			Msg::AnimDone => {
				let el = self.number_ref.cast::<HtmlElement>().unwrap();
				let classes = el.class_list();

				if classes.contains("number-spin-in") {
					classes.remove_1("number-spin-in").unwrap();
					self.member_count += 1;
				} else if classes.contains("number-spin-out") {
					classes.remove_1("number-spin-out").unwrap();
					self.member_count -= 1;
				}
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
			<div ref={self.body_ref.clone()} class="club-card">
				<div class="club-card-header">
					<h1>{self.props.details.unwrap().name.clone()}</h1>
				</div>
				<hr/>
				<div class="club-card-body">
					<div id="left-col">
						<h2><div ref=self.number_ref.clone() id="member-number">{self.member_count}</div> { if self.member_count == 0 || self.member_count > 1 {" members"} else { " member" }}</h2>
					</div>

					<div id="right-col">
						<img src={format!("/assets/clubs/{}.png", self.props.details.unwrap().id)}/>
						<p>{"Organizer"}</p>
						<h2>{format!("{} {}", self.props.details.unwrap().head_moderator.first_name, self.props.details.unwrap().head_moderator.last_name)}</h2>
					</div>
				</div>

				<div class="club-card-action-bar">
					<button id="club-card-join-btn" onclick={match self.which_button { JoinButton::FilledStar => leave_club, _ => join_club }}> <abbr data_title="Join">{self.which_button}</abbr> </button>
					<button id="club-card-expand-btn"><abbr data_title="View details"><span class="material-icons">{"open_in_full"}</span></abbr></button>

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
		}
	}

	fn rendered(&mut self, first: bool) {
		if first {
			let el = self.number_ref.cast::<HtmlElement>().unwrap();
			let link_clone = self.link.clone();

			let cb = Closure::wrap(Box::new(move || {
				link_clone.send_message(Msg::AnimDone);
			}) as Box<dyn Fn()>);

			el.set_ontransitionend(Some(cb.as_ref().unchecked_ref()));
			self.number_spin_anim_end_cb = Some(cb);

			let body = self.body_ref.cast::<HtmlElement>().unwrap();
			body.style().set_property("animation", format!("reveal-cards 0.3s linear {}s forwards", self.props.reveal_delay).as_str()).unwrap();
		}
	}
}
