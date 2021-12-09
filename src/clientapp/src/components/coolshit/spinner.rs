use yew::prelude::*;

pub struct Spinner {
	link: ComponentLink<Self>,
	props: Props,
}

#[derive(Clone, PartialEq)]
pub enum WhichSpinner {
	Ring1,
	Ring2,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	#[prop_or(WhichSpinner::Ring1)]
	pub which_spinner: WhichSpinner,
}

impl Component for Spinner {
	type Message = ();
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link, props }
	}

	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		match &self.props.which_spinner {
			WhichSpinner::Ring1 => html! {
				<div class="spinner"/>
			},

			WhichSpinner::Ring2 => html! {
				<div class="lds-dual-ring"/>
			},
		}
	}
}
