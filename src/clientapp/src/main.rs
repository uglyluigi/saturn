use yew::prelude::*;
mod components;
use components::{
    home::Home, notfound::NotFound, pg_login::LoginPageComponent, stellar::StellarBg,
    toolbar::ToolbarComponent,
};
use wasm_bindgen::prelude::*;
use web_sys::console::log_2;
use yew::{html::IntoPropValue, web_sys::Url};
use yew_router::{components::RouterAnchor, prelude::*, switch::Permissive};

mod router;
use router::*;

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
                html! { 
                    <div>
                        <StellarBg/>
                        <LoginPageComponent/> 
                    </div>
                }
            }
            AppRoute::Home => {
                html! { <Home/> }
            }
            AppRoute::NotFound(Permissive(route)) => {
                html! { <NotFound route=route /> }
            }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
