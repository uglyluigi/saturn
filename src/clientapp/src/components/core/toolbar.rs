use web_sys::HtmlElement;
use yew::{Component, ComponentLink, Html, NodeRef, Properties, ShouldRender, html, services::{FetchService, fetch::{FetchTask, Request, Response, StatusCode}}};
use crate::components::core::router::*;

pub struct ToolbarComponent {
	link: ComponentLink<Self>,
	props: Props,
	dropdown_content_ref: NodeRef,
	logout_task: Option<FetchTask>,
	redirect: bool,
}

pub enum Msg {
	ToggleDropdownState,
	SignOut,
	SignOutDone,
	SignOutFailed,
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
		logout_task: None, redirect: false }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::ToggleDropdownState => {
				let el = self.dropdown_content_ref.cast::<HtmlElement>().unwrap();
				
				let name = "dropdown-content-transition";

				if el.class_list().contains(name) {
					el.class_list().remove_1(name).unwrap();
				} else {
					el.class_list().add_1(name).unwrap();
				}
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
		};

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		let on_dropdown_button_clicked = self.link.callback(|e| {
			Msg::ToggleDropdownState
		});

		let on_signout_button_clicked = self.link.callback(|e| {
			Msg::SignOut
		});

		html! {
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
									<button class="dropdown-btn" onclick=on_dropdown_button_clicked>
										<img class="toolbar-pfp" src=self.props.pfp_url.clone()/>
										<h1>{"Hi, "} {self.props.username.clone()}</h1>
									</button>

									<div ref=self.dropdown_content_ref.clone() class="dropdown-content">
										<button class="normal-button" onclick=on_signout_button_clicked>{"Sign out"}</button>
									</div>
								</div>
							</>
						}
					}
				}
			</div>
		}
	}
}
