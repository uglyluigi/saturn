use crate::prelude::*;
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, AsChangeset)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub picture: String,
    pub first_name: String,
    pub last_name: String,
    pub is_admin: bool,
}

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub picture: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub is_admin: &'a bool,
}

#[derive(Serialize, Deserialize)]
pub struct Admin(pub User);