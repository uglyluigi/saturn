use crate::prelude::*;

#[delete("/clubs/<id>", rank=1)]
pub async fn delete_admin(_admin: Admin, db: Db, id: i32) -> std::result::Result<status::Accepted<()>, status::Custom<Option<Json<JsonError>>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, club_id};
    let club = Club::get_by_id_async(&db, &id).await;
    if club.is_some(){
        db.run(move |conn| {
            diesel::delete(club_members.filter(club_id.eq(id)))
                    .execute(conn)
                    .expect("Couldn't delete clubs_members prior to club deletion from database.");
            diesel::delete(clubs.find(id))
                .execute(conn)
                .expect("Couldn't delete clubs from database.");
        }).await;
        Ok(status::Accepted(None))
    }else {
        Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "The club you are trying to delete does not exist.".to_owned()}))))
    }
}

#[delete("/clubs/<id>", rank=2)]
pub async fn delete_user(user: User, db: Db, id: i32) -> std::result::Result<status::Accepted<()>, status::Custom<Option<Json<JsonError>>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, club_id};

    match user.get_membership_status_async(&db, &id).await {
        MembershipStatus::Moderator(is_head) => {
            if is_head {
                let _result = db.run(move |conn| {
                    diesel::delete(club_members.filter(club_id.eq(id)))
                        .execute(conn)
                        .expect("Couldn't delete clubs_members prior to club deletion from database.");
                    diesel::delete(clubs.find(id))
                        .execute(conn)
                        .expect("Couldn't delete clubs from database.");
                }).await;
                Ok(status::Accepted(None))
            } else {
                Err(status::Custom(Status::Forbidden, Some(Json(JsonError {error: "User is not a head moderator.".to_owned()}))))
            }
        },
        _ => {
            Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "User isn't even a moderator.".to_owned()}))))
        }
    }
}