use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use gloo_timers::callback::Timeout;
use regex::internal::Inst;
use wasm_bindgen::prelude::Closure;
use web_sys::HtmlElement;
use yew::{agent::Dispatcher, prelude::*, Properties};

use crate::{
	components::clubs::ClubView,
	event::{Amogus, EventBus},
	types::ClubDetails,
};

// TODO Future improvements
// Add search filters, aka searching specifically by user, body text, or organizer
// Obviously tags should be added too
pub struct SearchBar {
	link: ComponentLink<Self>,
	props: Props,
	search_field_state: Option<String>,
	emitter: ClubViewEmitter,
	delayed_search_cb: Option<Timeout>,
	toolbar_link: Dispatcher<EventBus>,
	search_bar_ref: NodeRef,
	container_ref: NodeRef,
}

pub struct ClubViewEmitter {
	show: bool,
}

// Container for keeping track of whether or not the club view should be shown.
// Remains hidden while the user is typing and for a little under a second after
// and automatically searches.
impl ClubViewEmitter {
	pub fn new(default: bool) -> Self {
		Self { show: default }
	}

	fn get(&self, search_text: Option<String>) -> Html {
		let f: fn(&String, &ClubDetails) -> bool = |filter, club| {
			let matcher = SkimMatcherV2::default();
			return matcher.fuzzy_match(&club.name, filter).is_some()
				|| matcher.fuzzy_match(&club.body, filter).is_some();
		};

		if self.show && search_text.is_some() {
			html! {
				<ClubView search_filter_function=crate::types::Mlk::new(Some(f)) search_filter_text=search_text/>
			}
		} else {
			html! {
				<>
				</>
			}
		}
	}

	pub fn set(&mut self, show: bool) {
		self.show = show;
	}
}

pub enum Msg {
	UpdateSearchFieldState(String),
	AfterKeyPress,
	SetEmitterState(bool),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	#[prop_or(None)]
	pub search_text: Option<String>,
}

impl Component for SearchBar {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		let mut search_text = props.search_text.clone();

		if let Some(str) = search_text {
			let s = str.replace("%20", " ").trim().to_owned();
			search_text = Some(s);
		}

		Self {
			link,
			emitter: ClubViewEmitter::new(search_text.is_some()),
			search_field_state: search_text,
			props,
			delayed_search_cb: None,
			toolbar_link: Amogus::dispatcher(),
			search_bar_ref: NodeRef::default(),
			container_ref: NodeRef::default(),
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::UpdateSearchFieldState(search) => {
				self.search_field_state = if search.len() > 0 { Some(search) } else { None };
			}

			Msg::AfterKeyPress => {
				if self.delayed_search_cb.is_some() {
					self.delayed_search_cb.take().unwrap().cancel();
				}

				self.link.send_message(Msg::SetEmitterState(false));
				let link = self.link.clone();

				let cb = Timeout::new(700, move || {
					let link = link;
					link.send_message(Msg::SetEmitterState(true));
				});

				self.delayed_search_cb = Some(cb);
			}

			Msg::SetEmitterState(b) => {
				self.emitter.set(b);
			}
		};
		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.props = props;
		true
	}

	fn view(&self) -> Html {
		let input_cb = self
			.link
			.callback(|data: yew::html::InputData| Msg::UpdateSearchFieldState(data.value));

		let key_cb = self.link.callback(|data: KeyboardEvent| Msg::AfterKeyPress);

		html! {
			<>
				<div ref=self.container_ref.clone() class="search-bar-container">
					<h1 class="search-bar-h1"> {"find something "} <i>{" totally "}</i> {" you."} </h1>
					<input ref=self.search_bar_ref.clone() class="search-bar-input" value=self.search_field_state.clone() onkeydown=key_cb oninput=input_cb placeholder="I'm looking for..."/>
				</div>

				{
					self.emitter.get(self.search_field_state.clone())
				}
			</>
		}
	}

	fn rendered(&mut self, first: bool) {
		if first {
			self.search_bar_ref
				.cast::<HtmlElement>()
				.unwrap()
				.focus()
				.unwrap();
			self.container_ref
				.cast::<HtmlElement>()
				.unwrap()
				.class_list()
				.add_1("search-bar-container-in")
				.unwrap();
			use crate::{
				components::core::toolbar::{Msg, WhichButton},
				event::*,
			};
			self.toolbar_link
				.send(Request::EventBusMsg(AgentMessage::ToolbarMsg(
					Msg::HighlightButton(WhichButton::Search),
				)));
		}
	}

	fn destroy(&mut self) {
		use crate::{
			components::core::toolbar::{Msg, WhichButton},
			event::*,
		};
		self.toolbar_link
			.send(Request::EventBusMsg(AgentMessage::ToolbarMsg(
				Msg::UnhighlightButton(WhichButton::Search),
			)))
	}
}
