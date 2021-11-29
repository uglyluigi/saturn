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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDetails{
    pub email: String,
    pub picture: String,
    pub first_name: String,
    pub last_name: String,
}

impl User{
    pub fn get_by_id(conn: &PgConnection, req_id: &i32) -> Option<User>{
        use crate::schema::users::dsl::{users, id};
        let value = users.filter(id.eq(req_id)).first(conn);

        if let Ok(value) = value {
            Some(value)
        }else{
            None
        }
    }

    pub async fn get_by_id_async(db: &Db, req_id: &i32) -> Option<User>{
        let req_id = req_id.clone();
        let result = db.run( move |conn| {
            Self::get_by_id(conn, &req_id)
        }).await;

        result
    }

    pub fn get_membership_status(self, conn: &PgConnection, club_id: &i32) -> MembershipStatus {
        use crate::schema::club_members::dsl::{club_members, club_id as db_club_id, user_id};
        let club_id = club_id.clone();
        let relation = club_members
            .filter(db_club_id.eq(club_id))
            .filter(user_id.eq(self.id))
            .limit(1).load::<ClubMember>(conn).unwrap();

        if relation.len() == 1 {
            if relation.get(0).unwrap().is_moderator == "head" {
                MembershipStatus::Moderator(true)
            } else if relation.get(0).unwrap().is_moderator == "true" {
                MembershipStatus::Moderator(false)
            } else {
                MembershipStatus::Member
            }
        } else {
            MembershipStatus::Unassociated
        }
    }

    pub async fn get_membership_status_async(self, db: &Db, club_id: &i32) -> MembershipStatus {
        let club_id = club_id.clone();
        let result = db.run(move |conn| {
            self.get_membership_status(conn, &club_id)
        }).await;

        result
    }

    pub fn to_user_details(&self) -> UserDetails{
        UserDetails{
            email: self.email.clone(),
            picture: self.picture.clone(),
            last_name: self.last_name.clone(),
            first_name: self.first_name.clone()
        }
    }
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