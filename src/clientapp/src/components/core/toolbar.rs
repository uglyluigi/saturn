use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::Closure;
use web_sys::{HtmlElement, MouseEvent};
use yew::{Component, ComponentLink, Html, NodeRef, Properties, ShouldRender, html, services::{FetchService, fetch::{FetchTask, Request, Response, StatusCode}}};
use crate::components::core::router::*;

pub struct ToolbarComponent {
	link: ComponentLink<Self>,
	props: Props,
	dropdown_content_ref: NodeRef,
	logout_task: Option<FetchTask>,
	redirect: bool,
	hide_timer: Option<Timeout>,
	is_signout_button: bool,
}

pub enum Msg {
	SignOut,
	SignOutDone,
	SignOutFailed,
	Hide,

	EnterSignOutButtonState,
	ExitSignOutButtonState,
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
		Self { link, props, dropdown_content_ref: NodeRef::default(),
		logout_task: None, redirect: false,
		hide_timer: None, is_signout_button: false, }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::EnterSignOutButtonState => {
				self.is_signout_button = true;

				let link = self.link.clone();

				self.hide_timer = Some(Timeout::new(3_000, move || {
					link.send_message(Msg::ExitSignOutButtonState);
				}));
			},

			Msg::ExitSignOutButtonState => {
				self.is_signout_button = false;
				self.hide_timer.take().unwrap().cancel();
			},

			Msg::SignOut => {
				let req = Request::post("/auth/logout").body(yew::format::Nothing);

				match req {
					Ok(req) => {
						let callback = self.link.callback(|response: Response<Result<String, anyhow::Error>>| {
							match response.status() {
								StatusCode::OK => {
									Msg::SignOutDone
								},

								_ => {
									Msg::SignOutFailed
								}
							}
						});

						match FetchService::fetch(req, callback) {
							Ok(task) => {
								self.logout_task = Some(task);
							},
							Err(_) => todo!(),
						};
					}

					Err(err) => (),
				}
			},

			Msg::SignOutFailed => {

			},

    		Msg::SignOutDone => {
				self.redirect = true;
			},

			Msg::Hide => {
				let name = "dropdown-content-transition";
				let el = self.dropdown_content_ref.cast::<HtmlElement>().unwrap();

				if el.class_list().contains(name) {
					el.class_list().remove_1(name).unwrap();
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
			Msg::EnterSignOutButtonState
		});

		let sign_out_cb = self.link.callback(|e: MouseEvent| {
			Msg::SignOut
		});

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
										<AppAnchor route=AppRoute::Search>{ "search" }</AppAnchor>
										<AppAnchor route=AppRoute::ClubForm>{ "add club" }</AppAnchor>
									</div>
								</div>
								
								<div class="toolbar-inner-component-right-side">
									<button class="dropdown-btn" onclick=if self.is_signout_button { on_dropdown_button_clicked } else { sign_out_cb }>
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
