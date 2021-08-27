use crate::prelude::*;
use crate::schema::club_members;

#[derive(Queryable, Serialize, Deserialize)]
pub struct ClubMember {
    pub id: i32,
    pub user_id: i32,
    pub club_id: i32,
    pub is_moderator: bool,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ClubMemberNullable {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub club_id: Option<i32>,
    pub is_moderator: Option<bool>,
}

#[derive(Insertable)]
#[table_name = "club_members"]
pub struct NewClubMember<'a> {
    pub user_id: &'a i32,
    pub club_id: &'a i32,
    pub is_moderator: &'a bool,
}