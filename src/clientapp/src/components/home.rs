use yew::prelude::*;
use crate::AppRedirect;
use crate::AppRoute;

pub struct Home {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link,  }
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
                <h1>{ "Home" } <AppRedirect route=AppRoute::Login/></h1>
            </div>
        }
    }
}