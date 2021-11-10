use yew::prelude::*;
use yew_router::{
	agent::RouteRequest,
	components::{Msg, Props, RouterAnchor},
	prelude::*,
	switch::Permissive,
};

use crate::types::ClubDetails;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
	#[to = "/login"]
	Login,
	#[to = "/page-not-found"]
	NotFound(Permissive<String>),
	#[to = "/test"]
	Test,
	#[to = "/!"]
	Home,
	#[to = "/search/{term}"]
	SearchTerm{term: String},
	#[to= "/search"]
	Search,
	#[to = "/new_club"]
	ClubForm,
	#[to = "/details/{id}"]
	Details{id: usize},

}

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;
pub type AppRedirect = RouterRedirect<AppRoute>;

pub struct RouterRedirect<SW: Switch + Clone + 'static, STATE: RouterState = ()> {
	link: ComponentLink<Self>,
	router: RouteAgentDispatcher<STATE>,
	props: Props<SW>,
}

impl<SW: Switch + Clone + 'static, STATE: RouterState> Component for RouterRedirect<SW, STATE> {
	type Message = Msg;
	type Properties = Props<SW>;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		let router = RouteAgentDispatcher::new();


		Self {
			link,
			router,
			props,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		let route: Route<STATE> = Route::from(self.props.route.clone());
		self.router.send(RouteRequest::ChangeRoute(route));
		false
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		self.link.send_message(Msg::Clicked);


		html! {
			<>
			</>
		}
	}

	fn rendered(&mut self, _first_render: bool) {

	}
}
