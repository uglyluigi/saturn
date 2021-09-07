use crate::prelude::*;

#[put("/clubs/<id>/renew")]
pub async fn renew(user: User, db: Db, id: i32) -> std::result::Result<Json<super::get::ClubDetails>, status::Custom<Option<Json<JsonError>>>> {
    use crate::schema::clubs::dsl::{clubs, expiry_date};

    let user_id=user.id.clone();
    match user.get_membership_status(&db, &id).await {
        MembershipStatus::Moderator => {
            let result = db.run(move |conn| {
                let update = diesel::update(clubs.find(id))
                    .set(expiry_date.eq(&(chrono::offset::Utc::now() + chrono::Duration::days(3))))
                    .get_result::<Club>(conn);
                
                if let Ok(update) = update{
                    let member = ClubMember{
                        id: -1,
                        user_id: user_id,
                        club_id: update.id,
                        is_moderator: true
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