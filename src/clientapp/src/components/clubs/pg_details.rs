use serde::{Deserialize, Serialize};
use yew::prelude::*;
use crate::{event::{self, Amogus, EventBus}, types::*};
use comrak::{ComrakExtensionOptions, ComrakOptions, arena_tree::Node, markdown_to_html};
#[macro_use]
use crate::tell;
use web_sys::{Blob, FileReader, HtmlElement, HtmlImageElement, HtmlInputElement, HtmlTextAreaElement};

pub struct DetailsPage {
	link: ComponentLink<Self>,
	props: Props,
	details: Option<ClubDetails>,
	msg_acceptor: Box<dyn yew::Bridge<Amogus>>,
	markdown_body_ref: NodeRef,
	markdown_rendered: bool
	
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
			markdown_body_ref: NodeRef::default(),
			markdown_rendered: false
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
								<div class="club-header">
									<div class="club-header-line">
										<h1 class="club-name">{details.name.clone()}</h1>
										<h1 class="club-edit"><abbr data_title="Edit"><span class="material-icons">{"edit"}</span></abbr></h1>
									</div>
									<h3>
										{"Published "} {date.format("%A, %B %e %Y")}
									</h3>
								</div>
								<div class="club-image-wrapper">
									<img class="club-image" src={format!("/assets/clubs/{}.png", details.id)}/>
									<div class="club-image-panel">
										<ul>
											<li>
												{"Created by "}<img src={format!("{}", details.head_moderator.picture)}/>
											</li>
											<li>
												{details.member_count} {" interested"}
											</li>
											<li>
												{"Published "} {date.format("%A, %B %e %Y")}
											</li>
										</ul>
									</div>
								</div>
								<div class="club-body">
									<hr/>
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
									
									<h2>{"About this Club"}</h2>
									<hr/>
									<div ref=self.markdown_body_ref.clone()>
									</div>
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

	fn rendered(&mut self, first_render: bool) {
		if self.details.is_some() && !self.markdown_rendered {
			let details = self.details.as_ref().unwrap();
			let mut date = details.publish_date;
		
			let el = self.markdown_body_ref.cast::<HtmlElement>().unwrap();
	
			el.set_inner_html(
				if let Some(md) = Some(details.body.clone()) {
					let md = markdown_to_html(
						md.as_str(),
						&ComrakOptions {
							extension: ComrakExtensionOptions {
								tagfilter: false,
								..ComrakExtensionOptions::default()
							},
							..ComrakOptions::default()
						},
					);
	
					self.markdown_rendered = true;
					ammonia::clean(md.as_str())
				} else {
					String::from("")
				}
				.as_str(),
			);
		}
    }
}
