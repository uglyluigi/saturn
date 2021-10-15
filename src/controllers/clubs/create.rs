use crate::prelude::*;

#[derive(Deserialize)]
pub struct NewClubDTO<'r> {
    pub name: Cow<'r, str>,
    pub body: Cow<'r, str>
}

#[post("/clubs/create", data = "<club>")]
pub async fn create(user: User, db: Db, club: Json<NewClubDTO<'_>>) -> Result<Json<ClubDetails>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members};

    let name = club.name.to_string().clone();
    let body = club.body.to_string().clone();
    let user_id = user.id.clone();

    let created_club: Club = db.run(move |conn| {
        let new_club = NewClub {
            name: &name.clone(),
            body: &body.clone(),
            publish_date: &chrono::offset::Utc::now(),
            expiry_date: &(chrono::offset::Utc::now() + chrono::Duration::days(3)),
        };

        let club = insert_into(clubs)
            .values(&new_club)
            .get_result::<Club>(conn).unwrap();


        let new_club_member = NewClubMember{
            user_id: &user.id,
            club_id: &club.id,
            is_moderator: &"head",
        };

        insert_into(club_members)
            .values(&new_club_member)
            .execute(conn).expect("Failed to add owner member to club.");
        
        club
    }).await;

    let member = ClubMember{
        id: -1,
        user_id: user_id,
        club_id: created_club.id,
        is_moderator: "head".to_owned()
    };

    Ok(Json(ClubDetails::from_join_async((member, created_club), user_id, db).await.unwrap()))
}
