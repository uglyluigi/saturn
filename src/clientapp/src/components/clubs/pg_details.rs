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
            <div class="details-page">

				{
					if self.details.is_some() {
						let details = self.details.as_ref().unwrap();
						let mut date = details.publish_date;

						html! {
							<>
								<img src={format!("/assets/clubs/{}.png", details.id)}/>
								<h1>{details.name.clone()}</h1>
								<h3>{details.member_count} {" interested"}</h3>
								<h3>{"Published "} {date.format("%A, %B %e %Y")}</h3>

								{
									if details.is_member {
										html! {
											<h3>{"You are interested in this club."}</h3>
										}
									} else {
										html! {
											<>
											</>
										}
									}
								}

								{
									if details.is_moderator == "head" {
										html! {
											<h3>{"You are the head moderator for this club."}</h3>
										}
									} else if details.is_moderator == "true" {
										html! {
											<h3>{"You are a moderator for this club."}</h3>
										}
									} else {
										html! {
											<>
											</>
										}
									}
								}
								
								<div>
									{details.body.clone()}
								</div>

								<div>
									
								</div>
							</>
						}
					} else {
						html! {
							<>
							</>
						}
					}
				}
            </div>
        }
	}
}
