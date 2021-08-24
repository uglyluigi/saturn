use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::service::RouteService;
use yew_router::route::RouteState;
use serde::{Serialize, Deserialize};


mod components;
use components::{pg_login::LoginPageComponent, toolbar::ToolbarComponent, stellar::StellarBg};

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    route_service: RouteService<SaturnRouteState>,
    route_state: SaturnRouteState,
}

#[derive(PartialEq, Serialize, Debug, Clone, Default, Deserialize)]
struct SaturnRouteState { }

#[derive(Debug, Switch, Clone)]
enum Route {
    #[to = "/"]
    Index,

    #[to = "/login"]
    Login,

    #[to = "/home"]
    Home,

    #[to = "/404"]
    NotFound
}

enum Msg {
    RouteChanged,
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, route_service: RouteService::<SaturnRouteState>::new(), route_state: SaturnRouteState::default() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged => true,
            _ => false
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<Route, SaturnRouteState> 
                render = Router::render(|route: Route| {
                    match route {
                        Route::Index => {
                            html! {
                                // todo: if not logged in...
                                // for now, redirect to login page
                                <div>
                                    <h1>{"Redirecting..."}</h1>
                                </div>
                            }
                        },

                        Route::Home => {
                            html!{"NYI"}
                        },

                        Route::Login => html! {
                            <div>
                                <StellarBg/>
                                <ToolbarComponent/>
                                <LoginPageComponent/>
                            </div>
                        },

                        Route::NotFound => html! {
                            <div>
                                <h1>{"404 Not Found :("}</h1>
                            </div>
                        }
                    }
                })
            />
        }
    }
}


fn main() {
    yew::start_app::<Model>();
}