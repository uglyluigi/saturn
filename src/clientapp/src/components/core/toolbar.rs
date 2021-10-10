use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use crate::components::core::router::*;

pub struct ToolbarComponent {
	link: ComponentLink<Self>,
	props: Props,
}

enum Msg {
	NowActive(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub username: String,
	pub pfp_url: String,
}

impl Component for ToolbarComponent {
	type Message = ();
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link, props }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div class="toolbar">
				<div class="toolbar-inner-component">	
					<AppAnchor route=AppRoute::Home><img id="logo" src="./assets/saturn-logo.svg"/></AppAnchor>
					<div class="toolbar-text-link">
						<AppAnchor route=AppRoute::Search>{ "search" }</AppAnchor>
						<AppAnchor route=AppRoute::ClubForm>{ "add club" }</AppAnchor>
					</div>
				</div>
				<div class="toolbar-inner-component-right-side">
					<img class="toolbar-pfp" src=self.props.pfp_url.clone()/>
					<h1>{"Hi, "} {self.props.username.clone()}</h1>
				</div>
			</div>
		}
	}
}
