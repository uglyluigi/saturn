use yew::{prelude::*, Properties};

use crate::components::{ClubCard, core::Toolbar};

pub struct ClubView {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub first_name: String,
    pub last_name: String,
}

impl Component for ClubView {
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
            <div class="club-view">
                <Toolbar username=self.props.first_name.clone()/>
                <ClubCard vote_count=0 organizer_name=String::from("Sans Undertale") club_name=String::from("Southeastern Undertale Club") club_description=String::from("The coolest club ever")/>
            </div>
        }
    }
}
