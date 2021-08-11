use crate::prelude::*;

#[get("/index")]
pub async fn index(db: Db) -> Result<Json<Vec<Club>>> {
    let clubs: Vec<Club> = db.run(move |conn| {
        clubs::table
            .load::<Club>(conn)
    }).await?;

    Ok(Json(clubs))
}