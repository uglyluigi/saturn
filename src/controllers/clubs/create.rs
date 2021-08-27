use crate::prelude::*;

#[derive(Deserialize)]
pub struct NewClubDTO<'r> {
    pub name: &'r str,
    pub body: &'r str
}

#[post("/clubs/create", data = "<club>")]
pub async fn create(user: User, db: Db, club: Json<NewClubDTO<'_>>) -> Result<Json<Club>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members};

    let name = club.name.to_string().clone();
    let body = club.body.to_string().clone();

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
            is_moderator: &true,
        };

        insert_into(club_members)
            .values(&new_club_member)
            .execute(conn).expect("Failed to add owner member to club.");
        
        club
    }).await;

    Ok(Json(created_club))
}
