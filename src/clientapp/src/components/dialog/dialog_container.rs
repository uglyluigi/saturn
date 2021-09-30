use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::components::*;

#[derive(Debug, Clone, PartialEq)]
pub enum DialogKind {
	NewClubDialog(types::Mlk<ComponentLink<ClubView>>),
}

pub enum CurrentAnimation {
	In,
	Out
}

pub enum Msg {
	Show,
	Close,
}

pub struct DialogContainer {
	link: ComponentLink<Self>,
	props: Props,
	current_animation: CurrentAnimation,
	hide: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
	pub kind: DialogKind,
	#[prop_or(None)]
	pub show: Option<bool>,
}

impl Component for DialogContainer {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link, props, current_animation: CurrentAnimation::In, hide: true }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		self.hide = false;

		match msg {
			Msg::Close => {
				self.current_animation = CurrentAnimation::Out;
				self.props.show = Some(false);
			},

			Msg::Show => {
				self.current_animation = CurrentAnimation::In;
			}
		};

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		if self.props != _props {
			self.props = _props;
		}
		true
	}

	fn rendered(&mut self, _first_render: bool) {
		if _first_render {
			self.hide = true;
		}
	}


	fn view(&self) -> Html {
		html! {
			<div style={if self.hide {String::from("display:none;")} else {String::new()}} class={if self.props.show.unwrap_or(true) {String::from("modal-bg-anim-in")} else { String::from("modal-bg-anim-out") }} id="modal-bg">
				{
					match self.props.kind.clone() {
						DialogKind::NewClubDialog(link) => {
							html! {
								<ClubDialog parent_link=link container_link=self.link.clone()/>
							}
						},
	
						_ => {
							html! {
								<>
								</>
							}
						}
					}
				}
			</div>
		}
	}
}
