use crate::prelude::*;
use crate::schema::club_members;

#[derive(Queryable, Serialize, Deserialize)]
pub struct ClubMember {
    pub id: i32,
    pub user_id: i32,
    pub club_id: i32,
    pub is_moderator: String,
}

#[derive(Insertable)]
#[table_name = "club_members"]
pub struct NewClubMember<'a> {
    pub user_id: &'a i32,
    pub club_id: &'a i32,
    pub is_moderator: &'a str,
}

pub enum MembershipStatus {
    Unassociated,
    Member,
    Moderator(bool)
}
