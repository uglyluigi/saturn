use crate::prelude::*;

#[get("/clubs/<id>", rank=1)]
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

#[get("/clubs/<id>", rank=2)]
pub async fn delete_user(user: User, db: Db, id: i32) -> Result<()> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, club_id, user_id, is_moderator};

    db.run(move |conn| {
        let relation = club_members
            .filter(club_id.eq(id))
            .filter(user_id.eq(user.id))
            .filter(is_moderator.eq(true))
            .limit(1).load::<ClubMember>(conn);

        

        if relation.is_ok() {
            diesel::delete(club_members.filter(club_id.eq(id)))
                .execute(conn)
                .expect("Couldn't delete clubs_members prior to club deletion from database.");
            diesel::delete(clubs.find(id))
                .execute(conn)
                .expect("Couldn't delete clubs from database.");
        }
    }).await;

    Ok(())
}