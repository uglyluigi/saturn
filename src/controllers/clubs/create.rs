use crate::prelude::*;

#[derive(Deserialize)]
pub struct NewClubDTO<'r> {
    pub name: Cow<'r, str>,
    pub body: Cow<'r, str>
}

#[post("/clubs/create", data = "<club>")]
pub async fn create(user: User, db: Db, club: Json<NewClubDTO<'_>>) -> Result<Json<Vec<ClubDetails>>, ()> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members};

    let name = club.name.to_string().clone();
    let body = club.body.to_string().clone();
    let user_id = user.id.clone();

    let (created_club, created_club_member): (Club, ClubMember) = db.run(move |conn| {
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

        let club_member = insert_into(club_members)
            .values(&new_club_member)
            .get_result::<ClubMember>(conn).expect("Failed to add owner member to club.");
        
        (club, club_member)
    }).await;

    match ClubDetails::from_join_async((created_club_member, created_club), user_id, db).await {
        Some(value) => {
            let mut vec = Vec::new();
            vec.push(value);
            Ok(Json(vec))
        },
        None => {
            eprintln!("Uh this wasn't supposed to happen.");
            //eprintln!("member {:?}, created_club {:?}, user_id {:?}, db {:?}",member,created_club,user_id,db);
            Err(())
        }
    }
}
