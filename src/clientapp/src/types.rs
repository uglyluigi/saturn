use std::fmt::{self, Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct UserDetails {
	pub email: String,
	pub picture: String,
	pub first_name: String,
	pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

// Wrapper type for components whose properties must derive PartialEq but must also
// store a ComponentLink to its parent. PartialEq is derived for props that may
// change; it stands to reason that the ComponentLink's state will not "change" in
// a way that should necessitate re-rendering the page at any point, so eq will
// always say that any PartialEqDummy<T> == PartialEqDummy<T>.

pub struct PartialEqDummy<T> {
	t: T,
}

impl<T> PartialEqDummy<T> {
	pub fn new(t: T) -> Self {
		Self {
			t
		}
	}

	pub fn unwrap(&self) -> &T {
		&self.t
	}

	pub fn unwrap_into(self) -> T {
		self.t
	}
}

use yew::ComponentLink;
use crate::components::ClubView;

impl<T> PartialEq for PartialEqDummy<T> {
	fn eq(&self, _: &PartialEqDummy<T>) -> bool {
		true
	}
}

impl<T: Clone> Clone for PartialEqDummy<T> {
	fn clone(&self) -> Self { 
		PartialEqDummy::new(self.t.clone())
	}
}

// All PartialEqDummy<T>'s are created equal (to other PartialEqDummy<T>'s)
pub type Mlk<T> = PartialEqDummy<T>;