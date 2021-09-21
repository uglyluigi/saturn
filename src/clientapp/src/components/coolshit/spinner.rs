use yew::prelude::*;

pub struct Spinner {
	link: ComponentLink<Self>,
}

impl Component for Spinner {
	type Message = ();
	type Properties = ();

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div class="spinner">
				<div class="cube1"></div>
				<div class="cube2"></div>
			</div>
		}
	}
}
