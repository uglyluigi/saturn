use serde::{Deserialize, Serialize};
use yew::prelude::*;
use crate::{event::{self, Amogus, EventBus}, types::*};

pub struct DetailsPage {
	link: ComponentLink<Self>,
	props: Props,
	details: Option<ClubDetails>,
	msg_acceptor: Box<dyn yew::Bridge<Amogus>>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub id: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Msg {
	AcceptDetails(ClubDetails),
	Ignore,
}

impl Component for DetailsPage {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {  
			props,
			details: None,
			msg_acceptor: EventBus::bridge(link.callback(|e| match e {
				event::AgentMessage::DetailsPageMsg(msg) => msg,
				_ => Msg::Ignore,
			})),
			link,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::AcceptDetails(details) => {
				self.details = Some(details)
			},
			Msg::Ignore => (),
		}

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
            <>
				<h1>{"Details for "}{self.props.id}</h1>

				{
					if self.details.is_some() {
						html! {
							<h3>{"Got details"}</h3>
						}
					} else {
						html! {
							<>
							</>
						}
					}
				}
            </>
        }
	}
}
