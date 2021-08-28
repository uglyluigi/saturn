use yew::{prelude::*, services::fetch::{FetchService, FetchTask, Request, Response}, format::{Nothing, Json}};
use web_sys::{MouseEvent, console::log_1};
use serde::{Deserialize};

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
    ReceieveUserInfo(User),
    FailToReceiveUserInfo(Option<anyhow::Error>),
}

macro_rules! tell {
    ($str_slice:expr) => (
        web_sys::console::log_1(&$str_slice.into())
    );

    ($str_slice:expr, $($arg:expr),*) => (
        tell!(format!($str_slice, $($arg),*))
    )
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
                    tell!("Got response");

                    if let Json(data) = response.into_body() {
                        match data {
                            Ok(user) => Msg::ReceieveUserInfo(user),
                            Err(err) => Msg::FailToReceiveUserInfo(Some(err)),
                        }
                    } else {
                        Msg::FailToReceiveUserInfo(None)
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
                self.fetch_task = None;
                tell!("User info received: {:?}", data);
            },

            Msg::FailToReceiveUserInfo(maybe_error) => {
                if let Some(error) = maybe_error {
                    self.fetch_task = None;
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
                <button onclick=callback>{"Button!"}</button>
                { self.fetch_user_info() }
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
                <></>
            }
        }
    }
}
