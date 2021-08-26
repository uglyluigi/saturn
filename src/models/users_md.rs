use crate::prelude::*;
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub is_admin: &'a bool,
}

#[derive(Serialize, Deserialize)]
pub struct Admin(pub User);