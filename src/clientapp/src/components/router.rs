use yew::prelude::*;
use yew_router::{components::{RouterAnchor, Props, Msg}, prelude::*, switch::Permissive, agent::RouteRequest};

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

        Self { link, router, props }
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

        html!{
            <>
            </>
        }
    }
}