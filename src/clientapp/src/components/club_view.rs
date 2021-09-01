use yew::prelude::*;
use crate::components::{toolbar::ToolbarComponent, router::*};
use yew_router::{prelude::*, switch::Permissive};
use std::rc::Rc;
use yew::Properties;

pub struct ClubView {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or(String::from("USERNAME_ERROR"))]
    pub username: String,
}

impl Component for ClubView {
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
                <ToolbarComponent username=self.props.username.clone()/>
            </div>
        }
    }
}