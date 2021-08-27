use crate::AppRedirect;
use crate::AppRoute;
use yew::{prelude::*, services::fetch::{FetchService, FetchTask, Request, Response}, format::{Nothing, Json}};
use anyhow::Error;
use serde::{Serialize, Deserialize};

extern crate yew;

pub struct Home {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub picture: String,
    pub first_name: String,
    pub last_name: String,
    pub is_admin: bool,
}

pub enum Msg {
    FetchUserInfo,
    ReceieveUserInfo(Result<User, anyhow::Error>),
}


impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, fetch_task: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchUserInfo => {
                let req = Request::get("/api/auth/details").body(Nothing).expect("Could not build request");
                
                let callback = self.link.callback(|response: Response<Json<Result<User, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    Msg::ReceieveUserInfo(data)
                });

                match FetchService::fetch(req, callback) {
                    Ok(task) => {
                        self.fetch_task = Some(task);
                    },

                    Err(err) => eprintln!("{}", err),
                }

                true
            },

            Msg::ReceieveUserInfo(data) => {
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                
            </div>
        }
    }
}

impl Home {
    fn fetch_user_info(&self) -> Html {
        if self.fetch_task.is_some() {
            html! {
                <h1> {"Loading..."} </h1>
            }
        } else {
            html! {
                html! {
                    <h1>{"Oh god! Oh Fuck!"} </h1>
                }
            }
        }
    }
}
