use yew::prelude::*;
mod components;
use components::{pg_login::LoginPageComponent, toolbar::ToolbarComponent, stellar::StellarBg};
use yew::{html::IntoPropValue, web_sys::Url};
use yew_router::{components::RouterAnchor, prelude::*, switch::Permissive};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/!"]
    Home,
    #[to = "/page-not-found"]
    NotFound(Permissive<String>),
}

impl AppRoute {
    pub fn into_public(self) -> PublicUrlSwitch {
        PublicUrlSwitch(self)
    }

    pub fn into_route(self) -> Route {
        Route::from(self.into_public())
    }
}

#[derive(Clone, Debug)]
pub struct PublicUrlSwitch(AppRoute);
impl PublicUrlSwitch {
    fn base_url() -> Url {
        if let Ok(Some(href)) = yew::utils::document().base_uri() {
            // since this always returns an absolute URL we turn it into `Url`
            // so we can more easily get the path.
            Url::new(&href).unwrap()
        } else {
            Url::new("/").unwrap()
        }
    }

    fn base_path() -> String {
        let mut path = Self::base_url().pathname();
        if path.ends_with('/') {
            // pop the trailing slash because AppRoute already accounts for it
            path.pop();
        }

        path
    }

    pub fn route(self) -> AppRoute {
        self.0
    }
}
impl Switch for PublicUrlSwitch {
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        if let Some(part) = part.strip_prefix(&Self::base_path()) {
            let (route, state) = AppRoute::from_route_part(part.to_owned(), state);
            (route.map(Self), state)
        } else {
            (None, None)
        }
    }

    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE> {
        route.push_str(&Self::base_path());
        self.0.build_route_section(route)
    }
}

impl IntoPropValue<PublicUrlSwitch> for AppRoute {
    fn into_prop_value(self: AppRoute) -> PublicUrlSwitch {
        self.into_public()
    }
}

pub type AppRouter = Router<PublicUrlSwitch>;
pub type AppAnchor = RouterAnchor<PublicUrlSwitch>;

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
                        AppRoute::NotFound(Permissive(Some(route.route))).into_public()
                    })
                />
            </div>
        }
    }
}

impl Model {
    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
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

fn main() {
    yew::start_app::<Model>();
}