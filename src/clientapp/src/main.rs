use yew::prelude::*;
mod components;
mod types;
mod flags;
use components::{
	coolshit::ThreeJSViewport,
	core::{router::*, NotFound},
	login::{Home, LoginPage},
	search::*

};
use yew_router::{prelude::*, switch::Permissive};

struct Model {
	link: ComponentLink<Self>,
}

impl Component for Model {
	type Message = ();
	type Properties = ();

	fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div>
				<AppRouter
					render=AppRouter::render(|thing : AppRoute| {
						Self::switch(thing)
					})

					redirect=AppRouter::redirect(|route: Route| {
						AppRoute::NotFound(Permissive(Some(route.route)))
					})
				/>
			</div>
		}
	}
}

impl Model {
	fn switch(switch: AppRoute) -> Html {
		match switch {
			AppRoute::Login => {
				html! { <LoginPage/> }
			}

			AppRoute::Home => {
				html! { <Home/> }
			}

			AppRoute::NotFound(Permissive(route)) => {
				html! { <NotFound route=route /> }
			}

			AppRoute::Test => {
				html! { <ThreeJSViewport/> }
			}

			AppRoute::Search => {
				html! {<SearchBar/>}
			}
		}
	}
}

fn main() {
	match yew::utils::host() {
		Ok(host) => {
			if host.starts_with("localhost") {
				tell!("Starting UI in test mode");
			}
		},
		Err(err) => tell!("Error getting host: {}", err),
	}

	yew::start_app::<Model>();
}
