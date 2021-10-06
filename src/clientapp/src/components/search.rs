use yew::{prelude::*, Properties};
use crate::types::*;
use crate::{
	components::core::{router::*, Toolbar}
};

pub struct SearchBar {
	link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
	DoZeTypeyType(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {

}

impl Component for SearchBar {
    type Message = Msg;
	type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
			Msg::DoZeTypeyType(search) => {

			},
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

    fn view(&self) -> Html {
        html!{
            <div class="search-bar-container">
                <h1 class="search-bar-h1"> {"find something "} <i>{" totally "}</i> {" you."} </h1>
                <input class="search-bar-input" placeholder="I'm looking for..."/>
            </div>
        }
    }
}
