use yew::prelude::*;
use yew_router::prelude::*;

use serde::{Serialize, Deserialize};
mod components;
use components::{pg_login::LoginPageComponent, toolbar::ToolbarComponent, stellar::StellarBg};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/"]
    Index,
    #[to = "/login"]
    Login
}

define_router_state!(crate::AppRoute)
use router_state::*;

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}

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
            <main>
                <Router
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Index => {
                                //Redir to login
                                html! {
                                    <h1>{"Redirecting..."}</h1>
                                }
                            },

                            AppRoute::Login => {
                                html! {
                                    <div>
                                        <StellarBg/>
                                        <LoginPageComponent/>
                                    </div>
                                }
                            }
                        }
                    })
                />
            </main>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}