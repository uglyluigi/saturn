use crate::prelude::*;

#[get("/clubs")]
pub async fn get_all(_user: User, db: Db) -> Result<Json<Vec<Club>>> {
    use crate::schema::clubs::dsl::{clubs};

    let loaded_clubs: Vec<Club> = db.run(move |conn| {
        clubs.load::<Club>(conn)
    }).await.expect("Couldn't retreive clubs from database.");

    Ok(Json(loaded_clubs))
}