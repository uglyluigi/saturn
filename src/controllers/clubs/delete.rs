use crate::prelude::*;

#[delete("/clubs/<id>", rank=1)]
pub async fn delete_admin(_admin: Admin, db: Db, id: i32) -> Result<()> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, club_id};

    db.run(move |conn| {
        diesel::delete(club_members.filter(club_id.eq(id)))
                .execute(conn)
                .expect("Couldn't delete clubs_members prior to club deletion from database.");
        diesel::delete(clubs.find(id))
            .execute(conn)
            .expect("Couldn't delete clubs from database.");
    }).await;

    Ok(())
}

#[delete("/clubs/<id>", rank=2)]
pub async fn delete_user(user: User, db: Db, id: i32) -> std::result::Result<status::Accepted<()>, status::Forbidden<()>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, club_id};

    match user.get_membership_status(&db, &id).await {
        MembershipStatus::Moderator => {
            let _result = db.run(move |conn| {
                diesel::delete(club_members.filter(club_id.eq(id)))
                    .execute(conn)
                    .expect("Couldn't delete clubs_members prior to club deletion from database.");
                diesel::delete(clubs.find(id))
                    .execute(conn)
                    .expect("Couldn't delete clubs from database.");
            }).await;
            Ok(status::Accepted(None))
        },
        _ => {
            Err(status::Forbidden(None))
        }
    }
}