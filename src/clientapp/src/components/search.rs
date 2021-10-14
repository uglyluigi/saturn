use yew::{prelude::*, Properties};
use crate::{components::{clubs::ClubView}, types::ClubDetails};

pub struct SearchBar {
	link: ComponentLink<Self>,
    props: Props,
    search_text: Option<String>,
    search_field_state: Option<String>,
    show_club_view: bool,
}

pub enum Msg {
	UpdateSearchFieldState(String),
    PerformSearch,
    Clear,
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
            props,
            search_text: None,
            search_field_state: None,
            show_club_view: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
			Msg::UpdateSearchFieldState(search) => {
                self.search_field_state = if search.len() > 0 { Some(search) } else { None };
			},


            Msg::PerformSearch => {
                self.show_club_view = true;
                self.search_text = self.search_field_state.clone();
            },

            Msg::Clear => {
                self.show_club_view = false;
            },
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.props = props;
        true
	}

    fn view(&self) -> Html {
        let input_cb = self.link.callback(|data: yew::html::InputData| {
            Msg::UpdateSearchFieldState(data.value)
        });

        let keypress_cb = self.link.callback(|e: yew::KeyboardEvent| {
            if e.key() == "Enter" {
                Msg::PerformSearch
            } else {
                Msg::Clear
            }
        });

        let f: fn(&String, &ClubDetails) -> bool = |filter, club| {
            club.name.contains(filter.as_str())
        };

        html!{
            <>
                <div class="search-bar-container">
                    <h1 class="search-bar-h1"> {"find something "} <i>{" totally "}</i> {" you."} </h1>
                    <input class="search-bar-input" oninput=input_cb value=self.search_field_state.clone() onkeypress=keypress_cb placeholder="I'm looking for..."/>
                </div>

                {
                    // FIXME I have to toggle this in order for it to refresh when you enter something new?? Not sure what's going on here.
                    if self.show_club_view {
                        html! {
                            <div>
                                {
                                    if self.search_text.is_some() {
                                        html! {
                                            <ClubView search_filter_function=crate::types::Mlk::new(Some(f)) search_filter_text=self.search_text.clone().unwrap()/>
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