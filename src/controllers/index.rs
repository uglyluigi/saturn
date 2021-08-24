use crate::prelude::*;

#[get("/index")]
pub async fn index(user: User, db: Db) -> Result<Json<Vec<Club>>> {
    use self::schema::clubs;
    let clubs: Vec<Club> = db.run(move |conn| {
        clubs::table
            .load::<Club>(conn)
    }).await?;

    Ok(Json(clubs))
}
