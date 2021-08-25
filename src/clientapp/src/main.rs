use yew::prelude::*;
mod components;
use components::{pg_login::LoginPageComponent, toolbar::ToolbarComponent, stellar::StellarBg};
use yew::{html::IntoPropValue, web_sys::Url};
use yew_router::{components::RouterAnchor, prelude::*, switch::Permissive};
use web_sys::console::log_2;
use wasm_bindgen::prelude::*;


#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/page-not-found"]
    NotFound(Permissive<String>),
    #[to = "/!"]
    Home,
}

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;
pub type AppRedirect = RouterRedirect<AppRoute>;

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                //<StellarBg/>
                //<ToolbarComponent/>
                //<LoginPageComponent/>
                <AppAnchor classes="navbar-item" route=AppRoute::Home>
                            { "Home" }
                </AppAnchor>
                <AppAnchor classes="navbar-item" route=AppRoute::Login>
                            { "Login" }
                </AppAnchor>
                <h1> {"Main Thing" } </h1>
                <AppRouter
                    render=AppRouter::render(Self::switch)
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
                html! { <crate::components::login::Login /> }
            }
            AppRoute::Home => {
                html! { <crate::components::home::Home /> }
            }
            AppRoute::NotFound(Permissive(route)) => {
                html! { <crate::components::notfound::NotFound route=route /> }
            }
        }
    }
}

use yew_router::components::{Props, Msg};
use yew_router::agent::RouteRequest;
pub struct RouterRedirect<SW: Switch + Clone + 'static, STATE: RouterState = ()> {
    
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<STATE>,
    props: Props<SW>,
}

impl<SW: Switch + Clone + 'static, STATE: RouterState> Component for RouterRedirect<SW, STATE> {
    type Message = Msg;
    type Properties = Props<SW>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        RouterRedirect {
            link,
            router,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        web_sys::console::log_2(&"%s : Hello World".into(),&"OnLoad".into());
        let route: Route<STATE> = Route::from(self.props.route.clone());
        self.router.send(RouteRequest::ChangeRoute(route));
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
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

fn main() {
    yew::start_app::<Model>();
}