use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct ClubDetails{
    pub id: i32,
    pub name: String,
    pub body: String,
    pub publish_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub is_member: bool,
    pub is_moderator: bool,
}

impl ClubDetails {
    fn from(join: (ClubMember, Club), user_id: i32) -> Self {
        ClubDetails {
            id: join.1.id,
            name: join.1.name,
            body: join.1.body,
            publish_date: join.1.publish_date,
            expiry_date: join.1.expiry_date,
            is_member: 
                join.0.user_id == user_id && 
                join.0.club_id==join.1.id,
            is_moderator: 
                join.0.user_id == user_id && 
                join.0.club_id==join.1.id &&
                join.0.is_moderator,
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
                is_moderator: false
            });
            results.push(ClubDetails::from((member_unwrapped, club), user.id));
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
            results.push(ClubDetails::from((member, club), user.id));
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
            .filter(is_moderator.eq(true))
            .load::<(ClubMember, Club)>(conn)
            .expect("Couldn't perform inner join with clubs from database.");
        
        let mut results = Vec::new();
        for (member, club) in join {
            results.push(ClubDetails::from((member, club), user.id));
        }
        
        results
    }).await;

    Ok(Json(loaded_clubs))
}