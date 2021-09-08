use crate::prelude::*;
use crate::schema::clubs;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Club {
    pub id: i32,
    pub name: String,
    pub body: String,
    pub publish_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>
}

#[derive(Insertable)]
#[table_name = "clubs"]
pub struct NewClub<'a> {
    pub name: &'a str,
    pub body: &'a str,
    pub publish_date: &'a DateTime<Utc>,
    pub expiry_date: &'a DateTime<Utc>
}