use yew::prelude::*;
use yew_router::prelude::*;


mod components;
use components::{pg_login::LoginPageComponent, toolbar::ToolbarComponent, stellar::StellarBg};

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}

#[derive(Debug, Switch, Clone)]
enum Route {
    #[to = "/"]
    Home,
    #[to = "/"]
    Secure
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
            <Router<Route> render={Router::render(|r: Route| {
                match r {
                    Route::Home => html!{ 
                        <div>
                            <StellarBg/>
                            <ToolbarComponent/>
                            <LoginPageComponent/> 
                        </div>
                    },
        
                    Route::Secure => html! {
                        {"Penis!"}
                    }
                }
            })} />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}