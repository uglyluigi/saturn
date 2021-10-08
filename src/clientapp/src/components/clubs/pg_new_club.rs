use anyhow::anyhow;
use serde_json::json;
use serde_json::Value;
use yew::format::Json;
use yew::services::fetch::{FetchTask, Request, Response, StatusCode};
use yew::services::FetchService;
use yew::{prelude::*, Html, ShouldRender};
use comrak::{markdown_to_html, ComrakOptions, ComrakExtensionOptions};


use crate::components::ClubView;
use crate::tell;
use crate::types::FetchState;

pub struct NewClubPage {
	link: ComponentLink<Self>,
	club_name_field_contents: Option<String>,
	club_body_field_contents: Option<String>,
	long_club_description_contents: Option<String>,
	props: Props,
	post_task: Option<FetchTask>,
	post_task_state: FetchState<()>,

	preview_div: Option<web_sys::Element>,
}

#[derive(Properties, Debug, Clone)]
pub struct Props {
}

pub enum Msg {
	Open,
	Close,
	Ignore,
	UpdateInfoState(WhichTextField, String),
	ValidateForm,

	PostClub,
	PostClubDone,
	UpdateClubs,
}

pub enum WhichTextField {
	TheNameOne,
	TheBodyOne,
	TheLongDescriptionOne,
}

impl NewClubPage {
	fn close(&self) {
	}

	fn reset(&mut self) {
		self.club_body_field_contents = None;
		self.club_name_field_contents = None;
		self.long_club_description_contents = None;
		self.post_task = None;
		self.post_task_state = FetchState::Waiting;
		self.preview_div = None;
	}

	fn get_preview(&self) -> Html {
		if let Some(val) = &self.long_club_description_contents {
			let md = markdown_to_html(val, &ComrakOptions {
				extension: ComrakExtensionOptions {
					tagfilter: false,
					..ComrakExtensionOptions::default()
				},
				..ComrakOptions::default()
			});
			let sanitized_md = ammonia::clean(md.as_str());
			self.preview_div.as_ref().unwrap().set_inner_html(sanitized_md.as_str());

			Html::VRef(self.preview_div.as_ref().unwrap().clone().into())
		} else {
			html! {
				<>
				</>
			}
		}
	}
}

impl Component for NewClubPage {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			club_name_field_contents: None,
			club_body_field_contents: None,
			long_club_description_contents: None,
			props,
			post_task: None,
			post_task_state: FetchState::Waiting,

			preview_div: None,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Open => (),
			Msg::Close => {
				self.reset();
			}
			Msg::Ignore => (),
			Msg::UpdateInfoState(which, value) => match which {
				WhichTextField::TheBodyOne => {
					self.club_body_field_contents = if value.len() > 0 { Some(value) } else { None }
				}

				WhichTextField::TheNameOne => {
					self.club_name_field_contents = if value.len() > 0 { Some(value) } else { None }
				}

				WhichTextField::TheLongDescriptionOne => {
					if self.preview_div.is_none() {
						self.preview_div = if let Ok(el) = yew::utils::document().create_element("div") {
							el.set_id("preview");
							Some(el)
						} else {
							None
						}
					}

					self.long_club_description_contents = if value.len() > 0 { Some(value) } else { None }
				}
			},

			Msg::ValidateForm => {
				// TODO
				self.link.send_message(Msg::PostClub);
			}

			Msg::PostClub => {
				self.post_task_state = FetchState::Waiting;

				if let (Some(name), Some(body)) = (
					self.club_name_field_contents.clone(),
					self.long_club_description_contents.clone(),
				) {
					let json = json!({"name": Value::String(name), "body": body.replace("\\", "\\\\")});
					let request = Request::post("/api/clubs/create")
						.body(Json(&json))
						.unwrap();

					let response_callback = self.link.callback(
						|response: Response<Json<Result<(), anyhow::Error>>>| {
							match response.status() {
								StatusCode::OK => {
									tell!("Successfully posted club");
									Msg::PostClubDone
								}

								_ => {
									tell!("Bad status receieved: {:?}", response.status());
									//Error stuff
									Msg::Ignore
								}
							}
						},
					);

					match FetchService::fetch(request, response_callback) {
						Ok(task) => self.post_task = Some(task),
						Err(err) => {
							tell!("Failed to post club: {}", err);
							self.post_task_state =
								FetchState::Failed(Some(anyhow!(format!("{:?}", err))));
						}
					}
				}
			}

			Msg::PostClubDone => {
				self.close();
				self.reset();
				self.link.send_message(Msg::UpdateClubs)
			},

			Msg::UpdateClubs => {
			}
		}

		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.props = props;
		true
	}

	fn view(&self) -> Html {
		let close_cb = self.link.callback(|_: MouseEvent| Msg::Close);

		let club_name_field_callback = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateInfoState(WhichTextField::TheNameOne, data.value)
		});

		let club_body_field_callback = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateInfoState(WhichTextField::TheBodyOne, data.value)
		});

		let long_description_field_callback = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateInfoState(WhichTextField::TheLongDescriptionOne, data.value)
		});

		html! {
            <div>
                <div class="new-club-page">
                    <h1>{"Create new club"}</h1>

					<input autocomplete="off" type="text" id="club-name-field" oninput=club_name_field_callback value=self.club_name_field_contents.clone() placeholder="Club Name"/>
					<div>
						<h2>{"Make your club stand out with some Markdown!"}</h2>
						<p>{"Type in the markdown editor below and describe your organization the way you like it!"}</p>
					</div>
                    <div>
                        <div id="description-and-preview-container">
							<h3>{"Body text"}</h3>
							<h3>{"Preview"}</h3>
                            <textarea id="long-club-description-input" value=self.long_club_description_contents.clone() oninput=long_description_field_callback/>
							{ 
								self.get_preview()
							}
                        </div>

                        
                    </div>

					<button onclick=self.link.callback(|_: MouseEvent| {Msg::ValidateForm})>{"OK"}</button>
                </div>
                
                <script>
                    {
                        stringify! {
                            document.getElementById("club-name-field").focus();
                        }
                    }
                </script>
            </div>
        } 
	}
}
