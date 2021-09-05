use yew::prelude::*;
use yew::Properties;

pub struct ClubCard {
    link: ComponentLink<Self>,
    props: Props,
    like_button_char: char,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub vote_count: i32,
    pub club_name: String,
    pub club_description: String,
    pub organizer_name: String,
}

pub enum Msg {
    ToggleLikeButton,
}

impl Component for ClubCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        //TODO like button is already toggled based on if the user liked this club
        Self {
            link,
            props,
            like_button_char: '💛',
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleLikeButton => {
                self.like_button_char = match self.like_button_char {
                    '💛' => '💖',
                    '💖' => '💛',
                    _ => panic!("WTF?"),
                };
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let callback = self
            .link
            .callback(|event: MouseEvent| Msg::ToggleLikeButton);

        html! {
            <div class="club-card">
                <div class="club-card-header">
                    <h1>{self.props.club_name.clone()}</h1>
                </div>
                <hr/>
                <div class="club-card-body">
                    <div id="left-col">
                        <h2>{self.props.vote_count.clone()} {" votes"}</h2>
                    </div>

                    <div id="right-col">
                        <img src="./assets/sans.jpg"/>
                        <p>{"Organizer"}</p>
                        <h2>{self.props.organizer_name.clone()}</h2>
                    </div>
                </div>
                <div class="club-card-action-bar">
                    <button onclick=callback>{self.like_button_char.clone()}</button>
                    <button>{"😂🍆👊💦😫"}</button>
                </div>
            </div>
        }
    }
}
