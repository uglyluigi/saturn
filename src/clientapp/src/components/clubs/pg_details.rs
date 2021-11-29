use serde::{Deserialize, Serialize};
use yew::{format::Json, prelude::*, services::fetch::{FetchTask, Response, StatusCode}};
use crate::{event::{self, Amogus, EventBus}, types::*};
use comrak::{ComrakExtensionOptions, ComrakOptions, arena_tree::Node, markdown_to_html};
<<<<<<< HEAD
use crate::components::core::router::*;
use crate::tell;
use web_sys::{Blob, FileReader, HtmlElement, HtmlImageElement, HtmlInputElement, HtmlTextAreaElement};
use yew_router::switch::Permissive;

=======
#[macro_use]
use crate::tell;
use web_sys::{Blob, FileReader, HtmlElement, HtmlImageElement, HtmlInputElement, HtmlTextAreaElement};
>>>>>>> 7cbeab4b01d135425480ad8ebe084c91cd9ece41

pub struct DetailsPage {
	link: ComponentLink<Self>,
	props: Props,
	details: Option<ClubDetails>,
	msg_acceptor: Box<dyn yew::Bridge<Amogus>>,
	markdown_body_ref: NodeRef,
<<<<<<< HEAD
	markdown_rendered: bool,
	
	get_details_task: Option<FetchTask>,
	get_details_task_state: FetchState<ClubDetails>,

	redirect_to_404: bool,
	redirect_to_logn: bool,
=======
	markdown_rendered: bool
	
>>>>>>> 7cbeab4b01d135425480ad8ebe084c91cd9ece41
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub id: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Msg {
	AcceptDetails(ClubDetails),
	Ignore,
	GetDetails,
	GetDetailsDone(ClubDetails),
	GetDetailsFail,
	RequestLogin,
}

impl Component for DetailsPage {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		link.send_message(Msg::GetDetails);

		Self {  
			props,
			details: None,
			msg_acceptor: EventBus::bridge(link.callback(|e| match e {
				event::AgentMessage::DetailsPageMsg(msg) => msg,
				_ => Msg::Ignore,
			})),
			link,
			markdown_body_ref: NodeRef::default(),
<<<<<<< HEAD
			markdown_rendered: false,
			get_details_task: None,
			get_details_task_state: FetchState::Waiting,

			redirect_to_404: false,
			redirect_to_logn: false,
=======
			markdown_rendered: false
>>>>>>> 7cbeab4b01d135425480ad8ebe084c91cd9ece41
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::AcceptDetails(details) => {
				self.details = Some(details);
			},
			
			Msg::GetDetailsDone(details) => {
				self.get_details_task = None;
				self.get_details_task_state = FetchState::Waiting;
				self.details = Some(details);
			},

			Msg::Ignore => (),
			Msg::GetDetails => {
				let req = yew::services::fetch::Request::get(format!("/api/clubs/{}", self.props.id))
					.body(yew::format::Nothing);

				self.get_details_task_state = FetchState::Waiting;

				match req {
					Ok(req) => {
						let callback = self.link.callback(
							|response: Response<Json<Result<ClubDetails, anyhow::Error>>>| {
								match response.status() {
									StatusCode::OK => {
										let Json(body) = response.into_body();

										match body {
											Ok(deets) => Msg::GetDetailsDone(deets),
											Err(err) => Msg::GetDetailsFail
										}
									}

									StatusCode::FORBIDDEN => Msg::RequestLogin,

									_ => {
										tell!(
											"Failed to get details: status code {}",
											response.status()
										);
										Msg::GetDetailsFail
									}
								}
							},
						);

						match yew::services::fetch::FetchService::fetch(req, callback) {
							Ok(task) => {
								self.get_details_task = Some(task);
							}
							Err(err) => {}
						}
					}

					Err(err) => {
						tell!("Failed to build request for user details: {:?}", err);
					}
				}
			},

			Msg::GetDetailsFail => {
				self.get_details_task_state = FetchState::Failed(None);
				self.get_details_task = None;
				self.redirect_to_404 = true;
			},

			Msg::RequestLogin => {
				self.redirect_to_logn = true;
			},
		}

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
            <div class="details-page">
				{
					if self.redirect_to_logn {
						html! {
							<AppRedirect route=AppRoute::Login/>
						}
					} else {
						html! {
							<>
							</>
						}
					}	
				}

				{
					if self.redirect_to_404 {
						html! {
							<AppRedirect route=AppRoute::NotFound(Permissive(Some("Mingus".to_owned())))/>
						}
					} else {
						html! {
							<>
							</>
						}
					}
				}

				{
					if self.details.is_some() {
						let details = self.details.as_ref().unwrap();
						let mut date = details.publish_date;

						html! {
							<>
								<div class="club-header">
									<div class="club-header-line">
										<h1 class="club-name">{details.name.clone()}</h1>
										<h1 class="club-edit"><abbr data_title="Edit"><span class="material-icons">{"edit"}</span></abbr></h1>
									</div>
									<h3>
										{"Published "} {date.format("%A, %B %e %Y")}
									</h3>
								</div>
								<div class="club-image-wrapper">
									<img class="club-image" src={format!("/assets/clubs/{}.png", details.id)}/>
									<div class="club-image-panel">
										<ul>
											<li>
												{"Created by "}<img src={format!("{}", details.head_moderator.picture)}/>
											</li>
											<li>
												{details.member_count} {" interested"}
											</li>
											<li>
												{"Published "} {date.format("%A, %B %e %Y")}
											</li>
										</ul>
									</div>
								</div>
								<div class="club-body">
									<hr/>
									{
										if details.is_member {
											html! {
												<h3>{"You are interested in this club."}</h3>
											}
										} else {
											html! {
												<>
												</>
											}
										}
									}

									{
										if details.is_moderator == "head" {
											html! {
												<h3>{"You are the head moderator for this club."}</h3>
											}
										} else if details.is_moderator == "true" {
											html! {
												<h3>{"You are a moderator for this club."}</h3>
											}
										} else {
											html! {
												<>
												</>
											}
										}
									}
									
									<h2>{"About this Club"}</h2>
									<hr/>
									<div ref=self.markdown_body_ref.clone()>
									</div>
								</div>
							</>
						}
					} else {
						html! {
							<>
							</>
						}
					}
				}
            </div>
        }
	}

	fn rendered(&mut self, first_render: bool) {
		if self.details.is_some() && !self.markdown_rendered {
			let details = self.details.as_ref().unwrap();
			let mut date = details.publish_date;
		
			let el = self.markdown_body_ref.cast::<HtmlElement>().unwrap();
	
			el.set_inner_html(
				if let Some(md) = Some(details.body.clone()) {
					let md = markdown_to_html(
						md.as_str(),
						&ComrakOptions {
							extension: ComrakExtensionOptions {
								tagfilter: false,
								..ComrakExtensionOptions::default()
							},
							..ComrakOptions::default()
						},
					);
	
					self.markdown_rendered = true;
					ammonia::clean(md.as_str())
				} else {
					String::from("")
				}
				.as_str(),
			);
		}
    }
}
