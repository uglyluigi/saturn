use yew::{prelude::*, Properties};
use crate::types::*;
use crate::components::ClubView;
use crate::tell;
use yew::services::fetch::{FetchService, FetchTask, Request, Response, StatusCode};


pub struct ClubCard {
	link: ComponentLink<Self>,
	props: Props,
	like_button_char: char,
	
	delete_button_char: char,
	delete_fetch_state: Option<FetchState<()>>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub id: i32,
	pub member_count: i64,
	pub club_name: String,
	pub club_description: String,
	pub organizer_name: String,
	#[prop_or(String::from("./assets/sans.jpg"))]
	pub organizer_pfp_url: String,
	pub parent_link: Mlk<ComponentLink<ClubView>>,
}

pub enum Msg {
	ToggleLikeButton,
	Delet,
	DoneDelet,
}

impl Component for ClubCard {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		//TODO like button is already toggled based on if the user liked this club
		Self {
			link,
			props,
			like_button_char: '💛',
			delete_button_char: '❌',
			delete_fetch_state: None,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::ToggleLikeButton => {
				self.like_button_char = match self.like_button_char {
					'💛' => '💖',
					'💖' => '💛',
					_ => panic!("WTF?"),
				};
			},

			Msg::Delet => {
				let req = Request::delete(format!("/api/clubs/delete/{}", self.props.id)).body(yew::format::Nothing);

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
							Ok(_) => (),
							Err(_) => (),
						}
					},
					Err(err) => (),
				}

				self.props.parent_link.unwrap().send_message(crate::components::club_view::Msg::GetClubDetails(None));
			},

			Msg::DoneDelet => {

			}
		}

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		let like = self.link.callback(|_: MouseEvent| Msg::ToggleLikeButton);
		let delete = self.link.callback(|_: MouseEvent| Msg::Delet);

		html! {
			<div class="club-card">
				<div class="club-card-header">
					<h1>{self.props.club_name.clone()}</h1>
				</div>
				<hr/>
				<div class="club-card-body">
					<div id="left-col">
						<h2>{self.props.member_count.clone()} {" members"}</h2>
					</div>

					<div id="right-col">
						<img src={self.props.organizer_pfp_url.clone()}/>
						<p>{"Organizer"}</p>
						<h2>{self.props.organizer_name.clone()}</h2>
					</div>
				</div>
				<div class="club-card-action-bar">
					<button onclick=like>{self.like_button_char.clone()}</button>
					
					{
						if let Some(state) = &self.delete_fetch_state {
							match state {
								FetchState::Done(_) => {
									html! {

									}
								},
								_ => {
									html! {
										<button disabled=true>{"..."}</button>
									}
								}
							}
						} else {
							html! {
								<button onclick=delete>{self.delete_button_char.clone()}</button>
							}
						}
					}
				</div>
			</div>
		}
	}
}
