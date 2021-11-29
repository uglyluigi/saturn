use crate::prelude::*;

#[get("/clubs")]
pub async fn get_all(user: User, db: Db) -> Result<Json<Vec<ClubDetails>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, club_id, user_id};

    let loaded_clubs: Vec<ClubDetails> = db.run(move |conn| {
        let club_load = clubs
            //.filter(user_id.eq(user.id).or(user_id.nullable().is_null()))
            .load::<Club>(conn)
            .expect("Couldn't perform left outer join with clubs from database.");
        
        let mut results = Vec::new();
        for  club  in club_load {
            let member = club_members.filter(user_id.eq(user.id)).filter(club_id.eq(club.id)).first::<ClubMember>(conn);

            let member_unwrapped = member.unwrap_or(ClubMember{
                id: -1,
                user_id: -1,
                club_id: -1,
                is_moderator: "false".to_owned()
            });
            if let Some(result)=ClubDetails::from_join((member_unwrapped, club), user.id, &conn){
                results.push(result)
            }
        }

        results
    }).await;

    Ok(Json(loaded_clubs))
}

#[get("/clubs/<id>")]
pub async fn get_club_details(user: User, db: Db, id: i32) -> std::result::Result<status::Custom<Json<ClubDetails>>, status::Custom<Option<Json<JsonError>>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, club_id, user_id};

    let loaded_clubs: Vec<ClubDetails> = db.run(move |conn| {
        let club_load = clubs
            .load::<Club>(conn)
            .expect("Couldn't perform left outer join with clubs from database.");
        
            let mut results = Vec::new();

            for  club  in club_load {
                if club.id == id {
                    let member = club_members.filter(user_id.eq(user.id)).filter(club_id.eq(club.id)).first::<ClubMember>(conn);
    
                    let member_unwrapped = member.unwrap_or(ClubMember{
                        id: -1,
                        user_id: -1,
                        club_id: -1,
                        is_moderator: "false".to_owned()
                    });
                    if let Some(result)=ClubDetails::from_join((member_unwrapped, club), user.id, &conn){
                        results.push(result)
                    }
                }
            }
    
            results
    }).await;

    if loaded_clubs.len() > 0 {
        Ok(status::Custom(Status::Ok, Json(loaded_clubs[0].to_owned())))
    } else {
        Err(status::Custom(Status::NotFound, Some(Json(JsonError {error: "The club you are trying to get the details of does not exist.".to_owned()}))))
    }
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
            if let Some(result)=ClubDetails::from_join((member, club), user.id, &conn){
                results.push(result);
            }
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
            if let Some(result)=ClubDetails::from_join((member, club), user.id, &conn){
                results.push(result);
            }
        }
        
        results
    }).await;

    Ok(Json(loaded_clubs))
}