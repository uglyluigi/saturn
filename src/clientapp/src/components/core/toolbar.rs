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

	HighlightButton(WhichButton),
	UnhighlightButton(WhichButton),
	AcceptExternalMsg,

	RevealDropdown,
	HideDropdown
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
			// Only accept toolbar messages
			// as long as a message is sent wrapped with ToolbarMsg this component
			// will respond to it.
			msg_acceptor: EventBus::bridge(link.callback(|e| match e {
				crate::event::AgentMessage::ToolbarMsg(msg) => msg,
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
				let name = "pfp-button-dropdown-in";
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
			},

			Msg::RevealDropdown => {
				let el = self.dropdown_content_ref.cast::<HtmlElement>().unwrap().class_list();

				if !el.contains("pfp-button-dropdown-in") {
					el.add_1("pfp-button-dropdown-in").unwrap();
					let link = self.link.clone();

					self.hide_timer = Some(Timeout::new(3_000, move || {
						link.send_message(Msg::HideDropdown);
					}));

				} else {
					self.link.send_message(Msg::HideDropdown)
				}

				
			},

			Msg::HideDropdown => {
				let el = self.dropdown_content_ref.cast::<HtmlElement>().unwrap().class_list();

				if el.contains("pfp-button-dropdown-in") {
					el.remove_1("pfp-button-dropdown-in").unwrap();
				}

				if self.hide_timer.is_some() {
					self.hide_timer.take().unwrap().cancel();
				}
			}
		};

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		let on_dropdown_button_clicked = self.link.callback(|e| {
			Msg::RevealDropdown
		});
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
									<AppAnchor route=AppRoute::Home><img id="logo" src="/assets/saturn-logo.svg"/></AppAnchor>
									<div class="toolbar-text-link">
										<AppAnchor ref=self.search_ref.clone() route=AppRoute::Search>{ "search" }</AppAnchor>
										<AppAnchor ref=self.add_club_ref.clone() route=AppRoute::ClubForm>{ "add club" }</AppAnchor>
									</div>
								</div>

								<div class="toolbar-inner-component-right-side">
									<button class="dropdown-btn" onclick=on_dropdown_button_clicked>
										<img class="toolbar-pfp" src=self.props.pfp_url.clone()/>
										<h1>
										{
											self.props.username.clone()
										}
										</h1>
									</button>
								</div>

								<div class="pfp-button-dropdown" ref=self.dropdown_content_ref.clone()>
										<button onclick=sign_out_cb>
											<span class="material-icons">
												{"exit_to_app"}
											</span>
											
											{"Sign out"}
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
