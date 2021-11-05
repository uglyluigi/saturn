use anyhow::anyhow;
use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions};

use serde_json::{json};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Blob, FileReader, HtmlElement, HtmlImageElement, HtmlInputElement};
use yew::{
	format::{Bincode, Json, Nothing},
	prelude::*,
	services::{
		fetch::{FetchTask, Request, Response, StatusCode},
		FetchService,
	},
	Html,
	ShouldRender,
};

use crate::{
	components::{ClubCard, Spinner},
	tell,
	types::{FetchState},
};

pub struct NewClubPage {
	link: ComponentLink<Self>,
	club_name_field_contents: Option<String>,
	club_body_field_contents: Option<String>,
	long_club_description_contents: Option<String>,
	club_logo_preview_src: Option<String>,
	props: Props,
	post_task: Option<FetchTask>,
	post_logo_task: Option<FetchTask>,

	post_task_state: FetchState<()>,
	post_logo_task_state: FetchState<()>,
	img_selector_ref: NodeRef,
	img_preview_ref: NodeRef,
	markdown_preview_ref: NodeRef,
	club_name_input_ref: NodeRef,
}

#[derive(Properties, Debug, Clone)]
pub struct Props {}

pub enum Msg {
	Ignore,
	UpdateInfoState(WhichTextField, String),
	UpdateClubLogoState(Vec<u8>),
	ValidateForm,
	PostClubLogo(i64),
	ReadLogo,

	PostClub,
	PostClubDone(i64),
}

pub enum WhichTextField {
	TheNameOne,
	TheBodyOne,
	TheLongDescriptionOne,
}

#[derive(Debug)]
pub enum ReadImgResult {
	TooBig,
	EmptyFileList,
}

impl NewClubPage {
	fn get_sanitized_md(&mut self) {}

	fn reset(&mut self) {
		self.club_body_field_contents = None;
		self.club_name_field_contents = None;
		self.long_club_description_contents = None;
		self.post_task_state = FetchState::Waiting;

		//self.img_preview_ref.cast::<HtmlImageElement>().unwrap().set_src("");
		self.markdown_preview_ref
			.cast::<HtmlElement>()
			.unwrap()
			.set_inner_html("");
	}

	fn read_img_sync(&self) {
		let file_reader = FileReader::new().expect("Unable to create file reader");
		let el = self.img_selector_ref.cast::<HtmlInputElement>().unwrap();

		if let Some(f) = el.files() {
			if let Some(file) = f.item(0) {
				let blob: &web_sys::Blob = file.as_ref();
				let link = self.link.clone();
				file_reader
					.read_as_data_url(&blob)
					.expect("Error reading image data");
				file_reader.set_onloadend(Some(
					Closure::once_into_js(move |x: ProgressEvent| {
						link.send_message(Msg::UpdateClubLogoState(
							x.target()
								.unwrap()
								.dyn_into::<FileReader>()
								.unwrap()
								.result()
								.unwrap()
								.as_string()
								.unwrap()
								.bytes()
								.collect(),
						))
					})
					.unchecked_ref(),
				));
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
			img_selector_ref: NodeRef::default(),
			img_preview_ref: NodeRef::default(),
			markdown_preview_ref: NodeRef::default(),
			club_name_input_ref: NodeRef::default(),
			post_logo_task: None,
			post_logo_task_state: FetchState::Waiting,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Ignore => (),
			Msg::UpdateInfoState(which, value) => match which {
				WhichTextField::TheBodyOne => {
					self.club_body_field_contents = if value.len() > 0 { Some(value) } else { None }
				}

				WhichTextField::TheNameOne => {
					self.club_name_field_contents = if value.len() > 0 { Some(value) } else { None }
				}

				WhichTextField::TheLongDescriptionOne => {
					self.long_club_description_contents =
						if value.len() > 0 { Some(value) } else { None };
					let el = self.markdown_preview_ref.cast::<HtmlElement>().unwrap();

					el.set_inner_html(
						if let Some(md) = &self.long_club_description_contents {
							let md = markdown_to_html(
								md.as_str(),
								&ComrakOptions {
									extension: ComrakExtensionOptions {
										tagfilter: false,
										..ComrakExtensionOptions::default()
									},
									..ComrakOptions::default()
								},
							);

							ammonia::clean(md.as_str())
						} else {
							String::from("")
						}
						.as_str(),
					);
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
					let json = json!({"name": name, "body": ammonia::clean(&body)});
					let request = Request::post("/api/clubs/create")
						.body(Json(&json))
						.unwrap();

					let response_callback = self.link.callback(
						|response: Response<Json<Result<(), anyhow::Error>>>| {
							match response.status() {
								StatusCode::OK => {
									tell!("Successfully post`ed club");
									tell!("{:?}", response);
									Msg::PostClubDone(20202)
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

			Msg::PostClubDone(id) => {
				crate::tell!("Mingus");
				self.reset();
				//self.link.send_message(Msg::PostClubLogo(id));
			}

			Msg::UpdateClubLogoState(bytes) => {
				let el = self.img_preview_ref.cast::<HtmlImageElement>().unwrap();

				match std::str::from_utf8(&bytes[0..]) {
					Ok(src) => el.set_src(src),
					Err(_) => crate::tell!("Something bad happen!"),
				}
			}

			Msg::PostClubLogo(id) => {}

			Msg::ReadLogo => {
				self.read_img_sync();
			}
		}
		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.props = props;
		true
	}

	fn view(&self) -> Html {
		let club_name_field_cb = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateInfoState(WhichTextField::TheNameOne, data.value)
		});

		let description_cb = self.link.callback(|data: yew::html::InputData| {
			Msg::UpdateInfoState(WhichTextField::TheLongDescriptionOne, data.value)
		});

		let submit_cb = self.link.callback(|cock| Msg::PostClub);

		let image_input_callback = self
			.link
			.callback(|data: yew::html::InputData| Msg::ReadLogo);

		html! {
			<div class="new-club-page">
				<div class="column col1">
					<h1>{"Create a new club!"}</h1>
					<div class="image-input">
						<img ref=self.img_preview_ref.clone() class="club-logo"/>
						<input ref=self.img_selector_ref.clone() oninput=image_input_callback type="file" name="file" id="file" class="inputfile"/>
						<label for="file">{"Select a club logo"}</label>
						<small>{"(png files only. <= 5MB in size)"}</small>
					</div>
					<input oninput=club_name_field_cb class="club-input" type="text" placeholder="Club name"/>
					<h3>{"Club description (markdown supported)"}</h3>
					<textarea oninput=description_cb class="markdown-textarea"/>
					<span>
						<button class="normal-button submit-new-club-button" onclick=submit_cb>{"Submit"}</button>
						{

							if self.post_task.is_some() {
								match self.post_task_state {
									FetchState::Waiting => html! {
										<>
											<h3>{"Submitting club..."}</h3>
											<Spinner/>
										</>
									},
									FetchState::Done(_) => html! {
										<>
											<h3>{"Done!"}</h3>
										</>
									},
									FetchState::Failed(_) => html! {
										<h3>{"Something bad happened."}</h3>
									},
								}
							} else {
								html! {
									<></>
								}
							}
						}
					</span>
				</div>

				<div class="column">
					<h1>{"Markdown preview"}</h1>
					<div ref=self.markdown_preview_ref.clone()>
					</div>
				</div>

			</div>
		}
	}

	fn rendered(&mut self, first: bool) {
		if first {
			//self.club_name_input_ref.cast::<HtmlElement>().unwrap().focus().unwrap();
		}
	}
}
