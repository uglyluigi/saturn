//Self
pub use crate::models::clubs_md::Club;
pub use crate::models::users_md::User;
pub use crate::models::users_md::NewUser;
pub use crate::Db;
pub use crate::Result;
pub use crate::schema;
pub use crate::UserAuthenticator;
//Self SB imports


//Rocket
pub use rocket::Rocket;
pub use rocket::Build;
pub use rocket::fairing::AdHoc;
pub use rocket::fs::FileServer;
pub use rocket::http::Status;
pub use rocket::response::{content, status};
pub use rocket::serde::{Serialize, Deserialize, json::Json};
pub use rocket::fs::relative;
pub use rocket::routes;
pub use rocket::figment::Figment;
pub use rocket::http::ContentType;
pub use rocket::response::Debug;
pub use rocket_sync_db_pools::{database, diesel};
pub use rocket::figment::{value::{Map, Value}, util::map};
pub use rocket::http::{Cookie, CookieJar};
pub use rocket::outcome::try_outcome;
pub use rocket::request::{self, Outcome, Request, FromRequest};
pub use std::collections::HashMap;
pub use rocket::config::SecretKey;
//Diesel
pub use diesel::prelude::*;
pub use diesel::pg::PgConnection;
pub use diesel::insert_into;
pub use chrono::{ DateTime, Utc };
//Other
pub use std::io::Cursor;
pub use dotenv::dotenv;
pub use std::env;
pub use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};