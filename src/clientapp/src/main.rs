use yew::prelude::*;
use yew_router::{route::Route, service::RouteService, Switch};
use yew::callback::Callback;

use serde::{Serialize, Deserialize};
mod components;
use components::{pg_login::LoginPageComponent, toolbar::ToolbarComponent, stellar::StellarBg};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/"]
    Index,
    #[to = "/login"]
    Login,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    service: RouteService<()>,
    route: Route<()>,
}

impl Model {
    fn change_route(&self, app_route: AppRoute) -> Callback<()> {
        self.link.callback(move |_| {
            let route = app_route.clone();
            Msg::ChangeRoute(route)
        })
    }
}

enum Msg {
    RouteChanged(Route<()>),
    ChangeRoute(AppRoute),
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut service = RouteService::<()>::new();
        let route = service.get_route();
        let callback = link.callback(Msg::RouteChanged);
        service.register_callback(callback);


        Self { link, service, route }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(r) => {
                self.route = r;
            },

            Msg::ChangeRoute(r) => {
                self.route = r.into();
                self.service.set_route(&self.route.route, ());
            },
        }

        true
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
                {
                    match AppRoute::switch(self.route.clone()) {
                        Some(AppRoute::Index) => {
                            self.link.send_message(Msg::ChangeRoute(AppRoute::Login));

                            html! {
                                { "Redirecting..." }
                            }
                        },

                        Some(AppRoute::Login) => {
                            html! {
                                <div>
                                    <StellarBg/>
                                    <LoginPageComponent/>
                                </div>
                            }
                        },

                        None => {
                            html! {
                                {"404"}
                         
                            }
                        }
                    }
                }
            </main>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}