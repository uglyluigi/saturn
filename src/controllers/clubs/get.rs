use crate::prelude::*;

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
            results.push(ClubDetails::from_join((member_unwrapped, club), user.id, &conn));
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
            results.push(ClubDetails::from_join((member, club), user.id, &conn));
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
            results.push(ClubDetails::from_join((member, club), user.id, &conn));
        }
        
        results
    }).await;

    Ok(Json(loaded_clubs))
}