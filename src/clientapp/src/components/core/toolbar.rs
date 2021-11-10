use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::Closure;
use web_sys::{HtmlElement, MouseEvent};
use yew::{Bridged, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender, html, services::{
		fetch::{FetchTask, Request, Response, StatusCode},
		FetchService,
	}};

use serde::{Serialize, Deserialize};

use crate::{components::core::router::*, event::{Amogus, EventBus}};


pub struct ToolbarComponent {
	link: ComponentLink<Self>,
	props: Props,
	dropdown_content_ref: NodeRef,
	logout_task: Option<FetchTask>,
	redirect: bool,
	hide_timer: Option<Timeout>,
	is_signout_button: bool,
	msg_acceptor: Box<dyn yew::Bridge<Amogus>>,

	search_ref: NodeRef,
	add_club_ref: NodeRef,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WhichButton {
	Search,
	AddClub,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Msg {
	SignOut,
	SignOutDone,
	SignOutFailed,
	Hide,
	DoNothing,

	EnterSignOutButtonState,
	ExitSignOutButtonState,
	HighlightButton(WhichButton),
	UnhighlightButton(WhichButton),
	AcceptExternalMsg,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub username: String,
	pub pfp_url: String,
}

impl Component for ToolbarComponent {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			props,
			dropdown_content_ref: NodeRef::default(),
			logout_task: None,
			redirect: false,
			hide_timer: None,
			is_signout_button: false,
			msg_acceptor: EventBus::bridge(link.callback(|e| match e {
				crate::event::AgentMessage::ToolbarMsg(msg) => {
					msg
				},

				_ => Msg::DoNothing,
			})),
			link, //I have to move this here because putting it as a field in a struct move it and I need to borrow it to make the message acceptor.
			search_ref: NodeRef::default(),
			add_club_ref: NodeRef::default(),
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::DoNothing => {},
			
			Msg::EnterSignOutButtonState => {
				self.is_signout_button = true;

				let link = self.link.clone();

				self.hide_timer = Some(Timeout::new(3_000, move || {
					link.send_message(Msg::ExitSignOutButtonState);
				}));
			}

			Msg::ExitSignOutButtonState => {
				self.is_signout_button = false;
				self.hide_timer.take().unwrap().cancel();
			}

			Msg::SignOut => {
				let req = Request::post("/auth/logout").body(yew::format::Nothing);

				match req {
					Ok(req) => {
						let callback = self.link.callback(
							|response: Response<Result<String, anyhow::Error>>| match response
								.status()
							{
								StatusCode::OK => Msg::SignOutDone,

								_ => Msg::SignOutFailed,
							},
						);

						match FetchService::fetch(req, callback) {
							Ok(task) => {
								self.logout_task = Some(task);
							}
							Err(_) => todo!(),
						};
					}

					Err(err) => (),
				}
			}

			Msg::SignOutFailed => {}

			Msg::SignOutDone => {
				self.redirect = true;
			}

			Msg::Hide => {
				let name = "dropdown-content-transition";
				let el = self.dropdown_content_ref.cast::<HtmlElement>().unwrap();

				if el.class_list().contains(name) {
					el.class_list().remove_1(name).unwrap();
				}
			},

			Msg::AcceptExternalMsg => {

			},

			Msg::HighlightButton(which) => {
				match which {
					WhichButton::AddClub => {
						self.add_club_ref.cast::<HtmlElement>().unwrap().class_list().add_1("selected").unwrap();
					},
					WhichButton::Search => {
						self.search_ref.cast::<HtmlElement>().unwrap().class_list().add_1("selected").unwrap();
					},
				}
			},

			Msg::UnhighlightButton(which) => {
				match which {
					WhichButton::AddClub => {
						self.add_club_ref.cast::<HtmlElement>().unwrap().class_list().remove_1("selected").unwrap();
					},
					WhichButton::Search => {
						self.search_ref.cast::<HtmlElement>().unwrap().class_list().remove_1("selected").unwrap();
					},
				}
			}
		};

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		let on_dropdown_button_clicked = self.link.callback(|e| Msg::EnterSignOutButtonState);
		let sign_out_cb = self.link.callback(|e: MouseEvent| Msg::SignOut);
		

		html! {
			<div class="toolbar-wrapper">
			<div class="toolbar">
				{
					if self.redirect {
						html! {
							<AppRedirect route=AppRoute::Login/>
						}
					} else {
						html! {
							<>
								<div class="toolbar-inner-component">
									<AppAnchor route=AppRoute::Home><img id="logo" src="./assets/saturn-logo.svg"/></AppAnchor>
									<div class="toolbar-text-link">
										<AppAnchor ref=self.search_ref.clone() route=AppRoute::Search>{ "search" }</AppAnchor>
										<AppAnchor ref=self.add_club_ref.clone() route=AppRoute::ClubForm>{ "add club" }</AppAnchor>
									</div>
								</div>

								<div class="toolbar-inner-component-right-side">
									<button class="dropdown-btn" onclick=if self.is_signout_button { sign_out_cb } else { on_dropdown_button_clicked }>
										<img class="toolbar-pfp" src=self.props.pfp_url.clone()/>
										<h1>
										{
											if self.is_signout_button {
												String::from("Sign out")
											} else {
												self.props.username.clone()
											}
										}
										</h1>
									</button>
								</div>
							</>
						}
					}
				}
				</div>
				<div class="login-content-border"></div>
			</div>
		}
	}
}
