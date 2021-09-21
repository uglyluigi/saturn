use std::fmt::{self, Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDetails {
	pub email: String,
	pub picture: String,
	pub first_name: String,
	pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClubDetails {
	pub id: i32,
	pub name: String,
	pub body: String,
	pub member_count: i64,
	pub publish_date: DateTime<Utc>,
	pub expiry_date: DateTime<Utc>,
	pub is_member: bool,
	pub is_moderator: String,
	pub head_moderator: UserDetails,
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

impl Default for AuthLevel {
	fn default() -> Self {
		AuthLevel::Guest
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthLevel {
	Admin,
	User,
	Guest,
}

impl Display for ClubDetails {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
		write!(f, "<></>")
	}
}

pub enum FetchState<T> {
	Waiting,
	Done(T),
	Failed(Option<anyhow::Error>),
}
