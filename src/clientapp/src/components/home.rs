use yew::{prelude::*, services::fetch::{FetchService, FetchTask, Request, Response}, format::{Nothing, Json}};
use web_sys::{MouseEvent, console::log_1};
use serde::{Deserialize, Serialize};

use super::router::*;
use crate::tell;

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
        Self { link, fetch_task: None, details: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchUserInfo => {
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

                                            Msg::ReceieveUserInfo(body)
                                        }

                                        _ => Msg::FailToReceiveUserInfo(Err("").into()),
                                    }
                                },

                                Err(err) => Msg::FailToReceiveUserInfo(Some(err.into())),
                            }
                        },
                        Err(err) => (),
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
                self.fetch_state = Some(FetchState::Failed());

                if let Some(error) = maybe_error {
                    tell!("Error getting user details: {:?}", error);
                }
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // Kind of silly that I have to provide a type annotation to something I'm ignoring!
        let callback = self.link.callback(|_: MouseEvent| {
            log_1(&"Fetching user data".into());
            Msg::FetchUserInfo
        });

        html! {
            <div>
                { self.check_auth() }

            </div>
        }
    }
}

impl Home {

    fn try_auth(&self) -> Result<AuthDetails, anyhow::Error> {
        
    }

    fn check_auth(&self) -> Html {
        
    }

    fn fetch_user_info(&self) -> Html {
        if self.fetch_task.is_some() {
            html! {
                <h1> {"Loading..."} </h1>
            }
        } else {
            html! {
                <></>
            }
        }
    }
}
