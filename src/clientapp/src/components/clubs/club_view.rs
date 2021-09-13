use yew::{prelude::*, Properties};

use crate::components::{core::router::*, core::Toolbar, ClubCard};

use anyhow::*;
use serde::{Deserialize, Serialize};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response, StatusCode},
};
use yew::{utils::document, web_sys::{Element, Node}, Html};
use yew::virtual_dom::VNode;


use crate::{tell, types::*};

enum FetchState {
    NotStarted,
    Waiting,
    Done,
    Failed,
}

use FetchState::*;

pub struct ClubView {
    link: ComponentLink<Self>,
    props: Props,
    redirect: Redirect,

    // Collections
    fetch_tasks: Vec<FetchTask>,
    clubs: Vec<ClubDetails>,

    // API Data
    user_details: Option<UserDetails>,
    auth_details: Option<AuthDetails>,

    clubs_fetch_state: FetchState,
    auth_details_fetch_state: FetchState,
    user_details_fetch_state: FetchState,

}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub first_name: String,
    pub last_name: String,
}

pub enum Msg {
    // Gets
    GetUserDetails,
    GetAuthDetails,
    GetClubDetails(Option<i32>),

    // Receives
    ReceiveUserDetails(Option<UserDetails>),
    ReceiveAuthDetails(Option<AuthDetails>),
    ReceiveClubDetails(Option<Vec<ClubDetails>>),

    // Other
    RequestLogin,
}

enum Redirect {
    No,
    Yes(AppRoute)
}

use Redirect::*;

impl ClubView {
    pub fn push_task(&mut self, task: FetchTask) {
        self.fetch_tasks.push(task);
    }

    pub fn clean_tasks(&mut self) {
        use yew::services::Task;
        let mut index: Option<usize> = None;

        for (i, e) in self.fetch_tasks.iter().enumerate() {
            if !e.is_active() {
                index = Some(i);
                break;
            }
        }

        if let Some(i) = index {
            drop(self.fetch_tasks.remove(i));
            self.clean_tasks();
        }
    }

    fn generate_club_list(&self) -> Html {
        let container: Element = document().create_element("div").unwrap();
        container.set_attribute("class", "club-list").unwrap();
        tell!("clubs = {:?}", self.clubs);

        for (i, club) in self.clubs.iter().enumerate() {
            match html! { <ClubCard vote_count=69 club_name=club.name.clone() club_description=club.body.clone() organizer_name=String::from("TODO")/> } {
                VNode::VRef(n) => tell!("Good!"),
                _ => tell!("Bad!!"),
            }
        }

        Html::VRef(container.into())
    }
}

impl Component for ClubView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            fetch_tasks: vec![],
            clubs: vec![],
            redirect: No,
            clubs_fetch_state: NotStarted,
            auth_details_fetch_state: NotStarted,
            user_details_fetch_state: NotStarted,
            auth_details: None,
            user_details: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        self.clean_tasks();

        match msg {
            GetUserDetails => {
                let req = yew::services::fetch::Request::get("/api/user/details")
                    .body(yew::format::Nothing);

                self.user_details_fetch_state = Waiting;

                match req {
                    Ok(req) => {
                        let callback = self.link.callback(
                            |response: Response<Json<Result<UserDetails, anyhow::Error>>>| {
                                match response.status() {
                                    StatusCode::OK => {
                                        tell!("Got user details");
                                        let Json(body) = response.into_body();

                                        match body {
                                            Ok(deets) => {
                                                ReceiveUserDetails(Some(deets))
                                            },
                                            Err(err) => {
                                                tell!("Failed to deser user data: {}", err);
                                                Msg::ReceiveUserDetails(None)
                                            }
                                        }
                                    }

                                    StatusCode::FORBIDDEN => Msg::RequestLogin,

                                    _ => {
                                        tell!(
                                            "Failed to receive user data: status code {}",
                                            response.status()
                                        );
                                        Msg::ReceiveUserDetails(None)
                                    }
                                }
                            },
                        );

                        match yew::services::fetch::FetchService::fetch(req, callback) {
                            Ok(task) => {
                                self.push_task(task);
                            }
                            Err(err) => {}
                        }
                    }

                    Err(err) => {
                        tell!("Failed to build request for user details: {:?}", err);
                    }
                }
            }

            GetAuthDetails => {
                let req = yew::services::fetch::Request::get("/api/auth/details")
                    .body(yew::format::Nothing);

                match req {
                    Ok(req) => {
                        let callback = self.link.callback(
                            |response: Response<Json<Result<UserDetails, anyhow::Error>>>| {
                                match response.status() {
                                    StatusCode::OK => {
                                        tell!("Got auth details");
                                        let Json(body) = response.into_body();

                                        match body {
                                            Ok(deets) => ReceiveUserDetails(Some(deets)),
                                            Err(err) => {
                                                tell!("Failed to deser auth data: {}", err);
                                                Msg::ReceiveUserDetails(None)
                                            }
                                        }
                                    },

                                    StatusCode::FORBIDDEN => {
                                        Msg::RequestLogin
                                    },

                                    _ => {
                                        tell!(
                                            "Failed to receive auth data: status code {}",
                                            response.status()
                                        );
                                        Msg::ReceiveUserDetails(None)
                                    }
                                }
                            },
                        );

                        match yew::services::fetch::FetchService::fetch(req, callback) {
                            Ok(task) => {
                                self.push_task(task);
                            }
                            Err(err) => {}
                        }
                    }

                    Err(err) => {
                        tell!("Failed to build request for auth details: {:?}", err);
                    }
                }
            }

            GetClubDetails(id) => {
                let req =
                    yew::services::fetch::Request::get("/api/clubs").body(yew::format::Nothing);

                match req {
                    Ok(req) => {
                        // TODO the response type in this callback is probably gonna have to change when all clubs are gotten from backend
                        let callback = self.link.callback(
                            |response: Response<Json<Result<Vec<ClubDetails>, anyhow::Error>>>| {
                                match response.status() {
                                    StatusCode::OK => {
                                        tell!("Got club details");
                                        let Json(body) = response.into_body();

                                        match body {
                                            Ok(deets) => ReceiveClubDetails(Some(deets)),
                                            Err(err) => {
                                                tell!("Failed to deser auth data: {}", err);
                                                Msg::ReceiveClubDetails(None)
                                            }
                                        }
                                    },

                                    StatusCode::FORBIDDEN => {
                                        Msg::RequestLogin
                                    },

                                    _ => {
                                        tell!(
                                            "Failed to receive auth data: status code {}",
                                            response.status()
                                        );
                                        Msg::ReceiveClubDetails(None)
                                    }
                                }
                            },
                        );

                        match yew::services::fetch::FetchService::fetch(req, callback) {
                            Ok(task) => {
                                self.push_task(task);
                            }
                            Err(err) => {}
                        }
                    }

                    Err(err) => {
                        tell!("Failed to build request for auth details: {:?}", err);
                    }
                }
            }

            ReceiveUserDetails(deets) => {
                if let Some(deet) = deets {
                    self.user_details_fetch_state = Done;
                    self.user_details = Some(deet);
                } else {
                    self.user_details_fetch_state = Failed;
                }
            }

            ReceiveAuthDetails(deets) => {
                if let Some(deet) = deets {
                    self.auth_details_fetch_state = Done;
                    self.auth_details = Some(deet);
                } else {
                    self.auth_details_fetch_state = Failed;
                }
            }

            ReceiveClubDetails(deets) => {
                if let Some(deet) = deets {
                    self.clubs_fetch_state = Done;
                    self.clubs = deet;
                } else {
                    self.clubs_fetch_state = Failed;
                }
            }

            RequestLogin => {
                self.redirect = Yes(AppRoute::Login);
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Yes(route) = &self.redirect {
            html! {
                <AppRedirect route=route.clone()/>
            }
        } else {
            html! {
                <div class="club-view">
                    <Toolbar username=self.props.first_name.clone()/>
                    <ClubCard vote_count=0 organizer_name=String::from("Sans Undertale") club_name=String::from("Southeastern Undertale Club") club_description=String::from("The coolest club ever")/>

                    { self.generate_club_list() }
                </div>
            }
        }
    }

    fn rendered(&mut self, first: bool) {
        if first {
            self.link.send_message(Msg::GetClubDetails(None));
        }
    }
}
