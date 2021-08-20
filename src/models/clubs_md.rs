use crate::prelude::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Club {
    pub id: i32,
    pub maintainer: i32,
    pub title: String,
    pub body: String,
    pub publish_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
}