use anyhow::*;
use serde::{Deserialize, Serialize};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response, StatusCode},
};

use crate::{
    components::{core::*, ClubView},
    please, tell,
    types::*
};

pub enum Msg {
    FetchUserInfo,
    ReceieveUserInfo(AuthDetails),
    FailToReceiveUserInfo(Option<anyhow::Error>),
}

pub struct Home {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    fetch_state: Option<FetchState>,
    details: Option<AuthDetails>,
}

pub enum FetchState {
    Waiting,
    Succeeded,
    Failed(Option<anyhow::Error>),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
            details: None,
            fetch_state: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchUserInfo => {
                self.fetch_state = Some(FetchState::Waiting);
                let req = Request::get("/api/auth/details").body(Nothing);

                match req {
                    Ok(req) => {
                        let callback = self.link.callback(
                            |response: Response<Json<Result<AuthDetails, anyhow::Error>>>| {
                                match response.status() {
                                    StatusCode::OK => {
                                        tell!("OK response");
                                        let Json(body) = response.into_body();

                                        match body {
                                            Ok(deets) => match deets.auth_level {
                                                Guest => Msg::FailToReceiveUserInfo(Some(anyhow!(
                                                    "Guests must log in"
                                                ))),
                                                _ => Msg::ReceieveUserInfo(deets),
                                            },
                                            Err(err) => Msg::FailToReceiveUserInfo(Some(err)),
                                        }
                                    }

                                    _ => Msg::FailToReceiveUserInfo(Some(anyhow!(
                                        "Weird status code received: {}",
                                        response.status()
                                    ))),
                                }
                            },
                        );

                        match FetchService::fetch(req, callback) {
                            Ok(task) => self.fetch_task = Some(task),
                            Err(err) => tell!("{}", err),
                        }
                    }

                    Err(err) => tell!("Error building request: {}", err),
                }
            }

            Msg::ReceieveUserInfo(data) => {
                self.fetch_state = Some(FetchState::Succeeded);
                self.fetch_task = None;
                tell!("User info received: {:?}", data);
            }

            Msg::FailToReceiveUserInfo(maybe_error) => {
                self.fetch_task = None;

                if let Some(error) = maybe_error {
                    tell!("Error getting user details: {:?}", error);
                    self.fetch_state = Some(FetchState::Failed(Some(error)));
                } else {
                    tell!("Unspecified error getting user details");
                    self.fetch_state = Some(FetchState::Failed(None));
                }
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let THIS_SHOULDNT_BE_TRUE = true;

        if THIS_SHOULDNT_BE_TRUE == true {
            html! {
                <ClubView first_name=String::from("DEBUG") last_name=String::from("GUY")/>
            }
        } else {
            self.normal_view()
        }
    }
}

impl Home {
    fn normal_view(&self) -> Html {
        match &self.fetch_state {
            Some(state) => {
                match state {
                    FetchState::Waiting => html! {
                        <h1> {"Waiting..."} </h1>
                    },

                    FetchState::Succeeded => html! {
                        //TODO handle token timeout. Just send Msg::RequestUserData again
                        <ClubView first_name=please!(self.details, first_name) last_name=please!(self.details, last_name)/>
                    },

                    FetchState::Failed(maybe_error) => {
                        match maybe_error {
                            Some(err) => tell!("Error: {:?}", err),
                            None => tell!("Unspecified error occurred."),
                        };

                        html! {
                            <AppRedirect route=AppRoute::Login/>
                        }
                    }
                }
            }

            None => {
                self.link.send_message(Msg::FetchUserInfo);

                html! {
                    <h1> {"Getting user data..."} </h1>
                }
            }
        }
    }
}
