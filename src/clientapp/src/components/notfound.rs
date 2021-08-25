use yew::prelude::*;

pub struct NotFound {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub route: Option<String>,
}

impl Component for NotFound {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1> {"Oh no!"} </h1>
            </div>
        }
    }
}
