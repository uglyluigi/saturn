use crate::prelude::*;

#[put("/clubs/<id>/renew")]
pub async fn renew(user: User, db: Db, id: i32) -> std::result::Result<Json<super::get::ClubDetails>, status::Unauthorized<String>> {
    use crate::schema::clubs::dsl::{clubs, expiry_date};
    use crate::schema::club_members::dsl::{club_members, club_id, user_id, is_moderator};

    let result = db.run(move |conn| {
        let relation = club_members
            .filter(club_id.eq(id))
            .filter(user_id.eq(user.id))
            .filter(is_moderator.eq(true))
            .limit(1).load::<ClubMember>(conn);

        if relation.is_ok() {
            let update = diesel::update(clubs.find(id))
                .set(expiry_date.eq(&(chrono::offset::Utc::now() + chrono::Duration::days(3))))
                .get_result::<Club>(conn)
                .expect("Couldn't update clubs in the database.");
            
            let member = ClubMember{
                id: -1,
                user_id: user.id,
                club_id: update.id,
                is_moderator: true
            };

            Some(super::get::ClubDetails::from_join((member, update), user.id))
        } else{
            None
        }
    }).await;

    match result {
        Some(club) => Ok(Json(club)),
        None => Err(status::Unauthorized(Some("User is not a club moderator.".to_string())))
    }
}