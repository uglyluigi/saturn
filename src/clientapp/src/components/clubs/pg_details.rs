use yew::prelude::*;
use crate::types::*;

pub struct DetailsPage {
	link: ComponentLink<Self>,
	props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub id: usize,
}

impl Component for DetailsPage {
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
            <>
				<h1>{"Details for mogus"}</h1>
            </>
        }
	}
}
