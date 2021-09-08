use crate::prelude::*;

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

impl ClubDetails {
    pub async fn from_join_with_db_pool(join: (ClubMember, Club), user_id: i32, db: Db) -> Self {
        use crate::schema::club_members::dsl::{club_members, club_id, is_moderator, user_id as club_members_user_id};

        let arg_club_id = join.1.id.clone();

        let member_count = db.run(move |conn| {
            club_members.filter(club_id.eq(arg_club_id)).count().first::<i64>(conn).unwrap()
        }).await;

        let user = db.run(move |conn| {
            let req_id = club_members.filter(club_id.eq(arg_club_id)).filter(is_moderator.eq("head")).select(club_members_user_id).first::<i32>(conn).unwrap();
            User::get_by_id(req_id, conn).unwrap()
        }).await;

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

    pub fn from_join_with_db_connection(join: (ClubMember, Club), user_id: i32, conn: &PgConnection) -> Self {
        use crate::schema::club_members::dsl::{club_members, club_id, is_moderator, user_id as club_members_user_id};

        let arg_club_id = join.1.id.clone();

        let member_count = club_members.filter(club_id.eq(arg_club_id)).count().first::<i64>(conn).unwrap();

        let req_id = club_members.filter(club_id.eq(arg_club_id)).filter(is_moderator.eq("head")).select(club_members_user_id).first::<i32>(conn).unwrap();
        let user = User::get_by_id(req_id, conn).unwrap();

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

#[get("/clubs")]
pub async fn get_all(user: User, db: Db) -> Result<Json<Vec<ClubDetails>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, user_id};

    let loaded_clubs: Vec<ClubDetails> = db.run(move |conn| {
        let join = clubs
            .left_outer_join(club_members)
            .filter(user_id.nullable().eq(user.id))
            .or_filter(user_id.nullable().is_null())
            .load::<(Club, Option<ClubMember>)>(conn)
            .expect("Couldn't perform left outer join with clubs from database.");
        
        let mut results = Vec::new();
        for (club, member) in join {
            let member_unwrapped = member.unwrap_or(ClubMember{
                id: -1,
                user_id: -1,
                club_id: -1,
                is_moderator: "false".to_owned()
            });
            results.push(ClubDetails::from_join_with_db_connection((member_unwrapped, club), user.id, &conn));
        }

        results
    }).await;

    Ok(Json(loaded_clubs))
}

#[get("/clubs/by/membership")]
pub async fn get_clubs_by_membership(user: User, db: Db) -> Result<Json<Vec<ClubDetails>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, user_id};

    let loaded_clubs: Vec<ClubDetails> = db.run(move |conn| {
        let join = club_members
            .inner_join(clubs)
            .filter(user_id.eq(user.id))
            .load::<(ClubMember, Club)>(conn)
            .expect("Couldn't perform inner join with clubs from database.");
        
        let mut results = Vec::new();
        for (member, club) in join {
            results.push(ClubDetails::from_join_with_db_connection((member, club), user.id, &conn));
        }

        results
    }).await;

    Ok(Json(loaded_clubs))
}

#[get("/clubs/by/moderatorship")]
pub async fn get_clubs_by_moderatorship(user: User, db: Db) -> Result<Json<Vec<ClubDetails>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, user_id, is_moderator};

    let loaded_clubs: Vec<ClubDetails> = db.run(move |conn| {
        let join = club_members
            .inner_join(clubs)
            .filter(user_id.eq(user.id))
            .filter(is_moderator.eq("true").or(is_moderator.eq("head")))
            .load::<(ClubMember, Club)>(conn)
            .expect("Couldn't perform inner join with clubs from database.");
        
        let mut results = Vec::new();
        for (member, club) in join {
            results.push(ClubDetails::from_join_with_db_connection((member, club), user.id, &conn));
        }
        
        results
    }).await;

    Ok(Json(loaded_clubs))
}