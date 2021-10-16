use anyhow::anyhow;
use serde_json::json;
use serde_json::Value;
use wasm_bindgen::JsCast;
use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::prelude::Closure;
use web_sys::HtmlImageElement;
use web_sys::HtmlInputElement;
use yew::format::Json;
use yew::services::fetch::{FetchTask, Request, Response, StatusCode};
use yew::services::FetchService;
use yew::{prelude::*, Html, ShouldRender};
use comrak::{markdown_to_html, ComrakOptions, ComrakExtensionOptions};
use web_sys::{FileReader, Blob};


use crate::components::ClubView;
use crate::tell;
use crate::types::FetchState;

pub struct NewClubPage {
	link: ComponentLink<Self>,
	club_name_field_contents: Option<String>,
	club_body_field_contents: Option<String>,
	long_club_description_contents: Option<String>,
	club_logo_preview_src: Option<String>,
	props: Props,
	post_task: Option<FetchTask>,
	post_task_state: FetchState<()>,
	img_selector_ref: NodeRef,
	img_preview_ref: NodeRef,



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
	UpdateClubLogoState(String),
	ValidateForm,

	PostClub,
	PostClubDone,
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
			club_logo_preview_src: None,
			props,
			post_task: None,
			post_task_state: FetchState::Waiting,
			preview_div: None,
			img_selector_ref: NodeRef::default(),
			img_preview_ref: NodeRef::default(),
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
					//FIXME back end often returns 422 on markdown with newlines and probably other stuff
																				// Clean your body with ammonia
					let json = json!({"name": json!(name), "body": json!(ammonia::clean(&body).replace("\n", "\\n"))});
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
				self.post_task = None;
			},

			Msg::UpdateClubLogoState(img) => {
				//TODO get rid of expect
				let file_reader = FileReader::new().expect("Unable to create file reader");
				let imgBlob = Blob::new().expect("Unable to create blob");
				let el = self.img_selector_ref.cast::<HtmlInputElement>().unwrap();
				let img_preview_element = self.img_preview_ref.cast::<HtmlImageElement>().unwrap();

				if let Some(f) = el.files() {
					let file = f.item(0).unwrap();
					let blob: &web_sys::Blob = file.as_ref();
					file_reader.read_as_data_url(&blob).expect("Error reading image data");

					file_reader.set_onloadend(Some(Closure::once_into_js( move |x: ProgressEvent| {
						let reader = x.target().unwrap().into_js_result().unwrap().dyn_into::<FileReader>().unwrap();
						img_preview_element.set_src(reader.result().unwrap().as_string().unwrap().as_str());						
					}).unchecked_ref()));

				}
			}
		}
		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.props = props;
		true
	}

	fn view(&self) -> Html {

		let club_name_field_callback = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateInfoState(WhichTextField::TheNameOne, data.value)
		});

		let long_description_field_callback = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateInfoState(WhichTextField::TheLongDescriptionOne, data.value)
		});

		let image_input_callback = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateClubLogoState(data.value)
		});

		html! {
            <div> 
                <div class="content new-club-page">
                    <h1>{"Create new club"}</h1>

					<div>
						<input autocomplete="off" type="text" id="club-name-field" oninput=club_name_field_callback value=self.club_name_field_contents.clone() placeholder="Club Name"/>
						<input ref=self.img_selector_ref.clone() type="file" id="club-logo-input" name="club-logo" accept="image/png" oninput=image_input_callback/>
						<img ref=self.img_preview_ref.clone() id="club-logo-preview" src=self.club_logo_preview_src.clone()/>
					</div>

					<div>
						<h2>{"Club Description"}<small>{" markdown supported"}</small></h2>
					</div>

                    <div>
                        <div id="description-and-preview-container">
							<span id="body-span">
								<h3>{"Body text"}</h3>
                            	<textarea value=self.long_club_description_contents.clone() oninput=long_description_field_callback/>
								<button onclick=self.link.callback(|_: MouseEvent| {Msg::ValidateForm})>{"Submit"}</button>

								{
									if self.post_task.is_some() {
										match &self.post_task_state {
											FetchState::Waiting => html! {
												{"Submitting club..."}
											},
				
											FetchState::Done(_) => html! {
												{"Done."}
											},
				
											FetchState::Failed(_) => html! {
												{"Failed."}
											}
										}
									} else {
										html! {
											<>
											</>
										}
									}
								}
							</span>

							<span id="preview-span">
								<h3>{"Preview"}</h3>

								{ 
									self.get_preview()
								}
							</span>
                        </div>

                        
                    </div>

					

                </div>
            </div>
        } 
	}
}
