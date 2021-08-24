use yew::prelude::*;
use serde::{Serialize, Deserialize};

#[macro_use]
extern crate yew_router;
use yew_router::prelude::*;

mod components;
use components::{pg_login::LoginPageComponent, toolbar::ToolbarComponent, stellar::StellarBg};

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}

#[derive(Debug, Switch, Clone, PartialEq, Serialize, Deserialize)]
enum AppRoute {
    #[to = "/"]
    Index,

    #[to = "/login"]
    Login,

    #[to = "/home"]
    Home,

    #[to = "/404"]
    NotFound
}

define_router_state!(Option<crate::AppRoute>);
use router_state::*;

enum Msg {
    RouteChanged,
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
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
            <Router<AppRoute, router_state::State> 
                render = Router::render(|route: AppRoute| -> Html {
                    match route {
                        AppRoute::Index => {
                            html! {
                                // todo: if not logged in...
                                // for now, redirect to login page
                                <div>
                                    <h1>{"Redirecting..."}</h1>
                                </div>
                            }
                        },

                        AppRoute::Home => {
                            html!{"NYI"}
                        },

                        AppRoute::Login => html! {
                            <div>
                                <StellarBg/>
                                <ToolbarComponent/>
                                <LoginPageComponent/>
                            </div>
                        },

                        AppRoute::NotFound => html! {
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