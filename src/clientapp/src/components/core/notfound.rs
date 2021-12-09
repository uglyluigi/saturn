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

	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div class="not-found">
				<img class = "logo" src ="assets/saturn-logo.svg" alt="Logo"/>
				<h1> {"404"} </h1>
				<h2> {"Page Not Found"} </h2>
				<h3> {"The page you're looking for either doesn't exist or an error occured. Go back to your momma"} </h3>
			</div>
		}
	}
}
