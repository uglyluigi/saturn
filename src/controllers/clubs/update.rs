use crate::prelude::*;

#[put("/clubs/<id>/renew")]
pub async fn renew(user: User, db: Db, id: i32) -> std::result::Result<Json<super::get::ClubDetails>, status::Custom<Option<Json<JsonError>>>> {
    use crate::schema::clubs::dsl::{clubs, expiry_date};

    let user_id=user.id.clone();
    match user.get_membership_status(&db, &id).await {
        MembershipStatus::Moderator(is_head) => {
            let result = db.run(move |conn| {
                let update = diesel::update(clubs.find(id))
                    .set(expiry_date.eq(&(chrono::offset::Utc::now() + chrono::Duration::days(3))))
                    .get_result::<Club>(conn);
                
                if let Ok(update) = update{
                    let member = ClubMember{
                        id: -1,
                        user_id: user_id,
                        club_id: update.id,
                        is_moderator: if is_head {"head".to_owned()} else {"true".to_owned()}
                    };
                    Ok(Json(super::get::ClubDetails::from_join_with_db_connection((member, update), user_id, &conn)))
                }else{
                    Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "The club you are trying to access does not exist.".to_owned()}))))
                }
            }).await;
            result
        },
        _ => {
            Err(status::Custom(Status::Forbidden, None))
        }
    }
}

#[put("/clubs/<id>/join")]
pub async fn join(user: User, db: Db, id: i32) -> std::result::Result<Json<super::get::ClubDetails>, status::Custom<Option<Json<JsonError>>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members};

    let user_id=user.id.clone();
    match user.get_membership_status(&db, &id).await {
        MembershipStatus::Unassociated => {
            let result = db.run(move |conn| {
                let club_exists = clubs.find(id).get_result::<Club>(conn);
                
                if club_exists.is_ok() {
                    let member = NewClubMember{
                        user_id: &user_id,
                        club_id: &id,
                        is_moderator: &"false",
                    };

                    let result = insert_into(club_members).values(member).get_result(conn);
                    Ok(Json(super::get::ClubDetails::from_join_with_db_connection((result.unwrap(), club_exists.unwrap()), user_id, &conn)))
                }else{
                    Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "The club you are trying to join does not exist.".to_owned()}))))
                }
            }).await;
            result
        },
        _ => {
            Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "User is already a member or moderator.".to_owned()}))))
        }
    }
}

#[put("/clubs/<id>/leave")]
pub async fn leave(user: User, db: Db, id: i32) -> std::result::Result<status::Accepted<()>, status::Custom<Option<Json<JsonError>>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, club_id, user_id};

    let user_id_copy = user.id.clone();
    match user.get_membership_status(&db, &id).await {
        MembershipStatus::Moderator(is_head) => {
            let result = db.run(move |conn| {
                let club_exists = clubs.find(id).get_result::<Club>(conn);
                
                if club_exists.is_ok() {
                    if !is_head{
                        let _result = diesel::delete(club_members).filter(club_id.eq(id)).filter(user_id.eq(&user_id_copy)).execute(conn);
                        Ok(status::Accepted(None))
                    }else{
                        Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "You are the appointed head of the club appoint a new one or delete the club".to_owned()}))))
                    }
                }else{
                    Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "The club you are trying to leave does not exist.".to_owned()}))))
                }
            }).await;
            result
        },
        MembershipStatus::Member => {
            let result = db.run(move |conn| {
                let club_exists = clubs.find(id).get_result::<Club>(conn);
                
                if club_exists.is_ok() {
                    let _result = diesel::delete(club_members).filter(club_id.eq(id)).filter(user_id.eq(&user_id_copy)).execute(conn);
                    Ok(status::Accepted(None))
                }else{
                    Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "The club you are trying to leave does not exist.".to_owned()}))))
                }
            }).await;
            result
        },
        _ => {
            Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "User is already unassociated with the club.".to_owned()}))))
        }
    }
}