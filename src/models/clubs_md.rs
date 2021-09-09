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

#[derive(Serialize, Deserialize)]
pub struct ClubDetails{
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

impl Club {
    pub fn to_club_details(self, conn: &PgConnection, user_id: &i32)  -> ClubDetails {
        ClubDetails::from_club(conn, self, user_id)
    }

    pub fn get_by_id(conn: &PgConnection, req_id: &i32) -> Option<Club>{
        use crate::schema::clubs::dsl::{clubs, id};
        let value = clubs.filter(id.eq(req_id)).first(conn);

        if let Ok(value) = value {
            Some(value)
        }else{
            None
        }
    }

    pub async fn get_by_id_async(db: &Db, req_id: &i32) -> Option<Club>{
        let req_id = req_id.clone();
        let result = db.run( move |conn| {
            Self::get_by_id(conn, &req_id)
        }).await;

        result
    }
}

impl ClubDetails {
    pub fn from_club(conn: &PgConnection, club : Club, user_id: &i32) -> Self{
        use crate::schema::club_members::dsl::{club_members, club_id, is_moderator, user_id as club_members_user_id};

        let member_count = club_members.filter(club_id.eq(club.id)).count().first::<i64>(conn).unwrap();

        let req_id = club_members.filter(club_id.eq(club.id)).filter(is_moderator.eq("head")).select(club_members_user_id).first::<i32>(conn).unwrap();
        let user = User::get_by_id(conn, &req_id).unwrap();
        let status = User::get_by_id(conn, user_id).unwrap()
            .get_membership_status(conn, &club.id);

        Self {
            id: club.id,
            name: club.name,
            body: club.body,
            publish_date: club.publish_date,
            expiry_date: club.expiry_date,
            member_count: member_count,
            is_member: 
                status == MembershipStatus::Member || 
                status == MembershipStatus::Moderator(false) ||
                status == MembershipStatus::Moderator(true),
            is_moderator: 
                if status == MembershipStatus::Moderator(true) {
                    "head".to_owned()
                } else if status == MembershipStatus::Moderator(false) {
                    "true".to_owned()
                } else { 
                    "false".to_owned() 
                },
            head_moderator:
                user.to_user_details()
        }
    }

    pub async fn from_join_async(join: (ClubMember, Club), user_id: i32, db: Db) -> Self {
        let result = db.run(move |conn| {
            Self::from_join(join, user_id, conn)
        }).await;

        result
    }

    pub fn from_join(join: (ClubMember, Club), user_id: i32, conn: &PgConnection) -> Self {
        use crate::schema::club_members::dsl::{club_members, club_id, is_moderator, user_id as club_members_user_id};

        let arg_club_id = join.1.id.clone();

        let member_count = club_members.filter(club_id.eq(arg_club_id)).count().first::<i64>(conn).unwrap();

        let req_id = club_members.filter(club_id.eq(arg_club_id)).filter(is_moderator.eq("head")).select(club_members_user_id).first::<i32>(conn).unwrap();
        let user = User::get_by_id(conn, &req_id).unwrap();

        ClubDetails {
            id: join.1.id,
            name: join.1.name,
            body: join.1.body,
            publish_date: join.1.publish_date,
            expiry_date: join.1.expiry_date,
            member_count: member_count,
            is_member: 
                join.0.user_id == user_id && 
                join.0.club_id==join.1.id,
            is_moderator: 
                if join.0.user_id == user_id && 
                join.0.club_id==join.1.id {
                join.0.is_moderator } else { "false".to_owned() },
            head_moderator:
                user.to_user_details()
        }
    }
}