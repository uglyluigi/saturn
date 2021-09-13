use yew::{prelude::*, Properties};

use crate::components::{ClubCard, core::Toolbar};

use anyhow::*;
use serde::{Deserialize, Serialize};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response, StatusCode},
};

use crate::{
    types::*,
    tell
};

pub struct ClubView {
    link: ComponentLink<Self>,
    props: Props,
    fetch_tasks: Vec<FetchTask>,
    clubs: Vec<ClubDetails>,
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
}

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
}

impl Component for ClubView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props, fetch_tasks: vec![], clubs: vec![] }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        self.clean_tasks();

        match msg {
            GetUserDetails => {
                let req = yew::services::fetch::Request::get("/api/user/details").body(yew::format::Nothing);
                match req {
                    Ok(req) => {
                        let callback = self.link.callback(
                            |response: Response<Json<Result<UserDetails, anyhow::Error>>>| {
                                match response.status() {
                                    StatusCode::OK => {
                                        tell!("Got user details");
                                        let Json(body) = response.into_body();

                                        match body {
                                            Ok(deets) => ReceiveUserDetails(Some(deets)),
                                            Err(err) => {
                                                tell!("Failed to deser user data: {}", err);
                                                Msg::ReceiveUserDetails(None)
                                            }
                                        }
                                    },

                                    _ => {
                                        tell!("Failed to receive user data: status code {}", response.status());
                                        Msg::ReceiveUserDetails(None)
                                    }
                                }
                            },
                        );

                        match yew::services::fetch::FetchService::fetch(req, callback) {
                            Ok(task) => {
                                self.push_task(task);
                            },
                            Err(err) => {}
                        }
                    },

                    Err(err) => {
                        tell!("Failed to build request for user details: {:?}", err);
                    }
                }
            },

            GetAuthDetails => {
                let req = yew::services::fetch::Request::get("/api/auth/details").body(yew::format::Nothing);

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

                                    _ => {
                                        tell!("Failed to receive auth data: status code {}", response.status());
                                        Msg::ReceiveUserDetails(None)
                                    }
                                }
                            },
                        );

                        match yew::services::fetch::FetchService::fetch(req, callback) {
                            Ok(task) => {
                                self.push_task(task);
                            },
                            Err(err) => {}
                        }
                    },

                    Err(err) => {
                        tell!("Failed to build request for auth details: {:?}", err);
                    }
                }
            },

            GetClubDetails(id) => {
                let req = yew::services::fetch::Request::get("/api/clubs").body(yew::format::Nothing);


                match req {
                    Ok(req) => {
                        // TODO the response type in this callback is probably gonna have to change when all clubs are gotten from backend
                        let callback = self.link.callback(
                            |response: Response<Json<Result<Vec<ClubDetails>, anyhow::Error>>>| {
                                match response.status() {
                                    StatusCode::OK => {
                                        tell!("Got auth details");
                                        let Json(body) = response.into_body();

                                        match body {
                                            Ok(deets) => ReceiveClubDetails(Some(deets)),
                                            Err(err) => {
                                                tell!("Failed to deser auth data: {}", err);
                                                Msg::ReceiveClubDetails(None)
                                            }
                                        }
                                    },

                                    _ => {
                                        tell!("Failed to receive auth data: status code {}", response.status());
                                        Msg::ReceiveClubDetails(None)
                                    }
                                }

                            },
                        );

                        match yew::services::fetch::FetchService::fetch(req, callback) {
                            Ok(task) => {
                                self.push_task(task);
                            },
                            Err(err) => {}
                        }
                    },

                    Err(err) => {
                        tell!("Failed to build request for auth details: {:?}", err);
                    }
                }
            },

            ReceiveUserDetails(deets) => {},
            ReceiveAuthDetails(deets) => {},
            ReceiveClubDetails(deets) => {
                if let Some(deet) = deets {
                    tell!("Got club deets: {:?}", deet);
                }
            },
        }

        true
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

    fn rendered(&mut self, first: bool) {
        if first {
            self.link.send_message(Msg::GetClubDetails(None));
        }
    }
}
