use yew::{prelude::*, services::fetch::{FetchService, FetchTask, Request, Response}, format::{Nothing, Json}};
use web_sys::{MouseEvent, console::log_1};
use serde::{Deserialize, Serialize};

use super::router::*;
use crate::tell;
use anyhow::*;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub picture: String,
    pub first_name: String,
    pub last_name: String,
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthLevel {
    Admin,
    User,
    Guest
}

impl Default for AuthLevel {
    fn default() -> Self { AuthLevel::Guest }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AuthDetails {
    pub auth_level: AuthLevel,
    pub id: Option<i32>,
    pub email: Option<String>,
    pub picture: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub enum Msg {
    FetchUserInfo,
    ReceieveUserInfo(AuthDetails),
    FailToReceiveUserInfo(Option<anyhow::Error>),
}


pub struct Home {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    fetch_state: Option<FetchState>,
    details: Option<AuthDetails>
}

pub enum FetchState {
    Waiting,
    Succeeded,
    Failed(Option<anyhow::Error>)
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, fetch_task: None, details: None, fetch_state: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchUserInfo => {
                self.fetch_state = Some(FetchState::Waiting);
                let req = Request::get("/api/auth/details").body(Nothing).expect("Could not build request");
                
                let callback = self.link.callback(|response: Response<Json<Result<AuthDetails, anyhow::Error>>>| {
                    let status = response.headers()["status"].to_str().unwrap();

                    tell!("Got response: {}", status);

                    match response.headers()["status"].to_str() {
                        Ok(code) => {
                            tell!("Response code: {}", code);

                            match code.parse::<i32>() {
                                Ok(code) => {
                                    match code {
                                        200 => {
                                            let Json(body) = response.into_body();

                                            match body {
                                                Ok(deets) => Msg::ReceieveUserInfo(deets),
                                                Err(err) => Msg::FailToReceiveUserInfo(Some(err))
                                            }
                                        }

                                        _ => Msg::FailToReceiveUserInfo(Some(anyhow!("Non-200 status received: {}", code))),
                                    }
                                },

                                Err(err) => Msg::FailToReceiveUserInfo(Some(err.into())),
                            }
                        },
                        Err(err) => Msg::FailToReceiveUserInfo(Some(err.into())),
                    }
                });

                match FetchService::fetch(req, callback) {
                    Ok(task) => {
                        self.fetch_task = Some(task);
                    },

                    Err(err) => tell!("{}", err),
                }
            },

            Msg::ReceieveUserInfo(data) => {
                self.fetch_state = Some(FetchState::Succeeded);
                self.fetch_task = None;
                tell!("User info received: {:?}", data);
            },

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
        match &self.fetch_state {
            Some(state) => {
                match state {
                    FetchState::Waiting => html! {
                        <h1> {"Waiting..."} </h1>
                    },

                    FetchState::Succeeded => html! {
                        //TODO handle token timeout. Just send Msg::RequestUserData again
                        <h1> {"You're home!"} </h1>
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
            },

            None => {
                self.link.callback(|_: Option<()>| {
                    tell!("Fetching user data");
                    Msg::FetchUserInfo
                }).emit(None);

                html! {
                    <h1> {"Getting user data..."} </h1>
                }
            }
        }
    }
}