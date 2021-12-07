use crate::prelude::*;

#[derive(Deserialize)]
pub struct UpdateClubDTO<'r> {
    pub name: Cow<'r, str>,
    pub body: Cow<'r, str>
}

#[put("/clubs/<id>", data = "<club>")]
pub async fn update(user: User, db: Db, id: i32, club: Json<UpdateClubDTO<'_>>) -> std::result::Result<Json<ClubDetails>, status::Custom<Option<Json<JsonError>>>> {
    let user_id=user.id.clone();
    use crate::schema::clubs::dsl::{clubs, name,body};

    let club_name = club.name.to_string().clone();
    let club_body = club.body.to_string().clone();
    match user.get_membership_status_async(&db, &id).await {
        MembershipStatus::Moderator(is_head) => {
            if is_head {
                let result = db.run(move |conn| {
                    let update = diesel::update(clubs.find(id))
                        .set((
                            name.eq(club_name),
                            body.eq(club_body),
                        ))
                        .get_result::<Club>(conn);
                    
                    if let Ok(update) = update{
                        let member = ClubMember{
                            id: -1,
                            user_id: user_id,
                            club_id: update.id,
                            is_moderator: if is_head {"head".to_owned()} else {"true".to_owned()}
                        };
                        Ok(Json(ClubDetails::from_join((member, update), user_id, &conn).unwrap()))
                    }else{
                        Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "The club you are trying to access does not exist.".to_owned()}))))
                    }
                }).await;
                result
            }else{
                Err(status::Custom(Status::Forbidden, None))
            }
        },
        _ => {
            Err(status::Custom(Status::Forbidden, None))
        }
    }
}


#[put("/clubs/<id>/renew")]
pub async fn renew(user: User, db: Db, id: i32) -> std::result::Result<Json<ClubDetails>, status::Custom<Option<Json<JsonError>>>> {
    use crate::schema::clubs::dsl::{clubs, expiry_date};

    let user_id=user.id.clone();
    match user.get_membership_status_async(&db, &id).await {
        MembershipStatus::Moderator(is_head) => {
            let result = db.run(move |conn| {
                let update = diesel::update(clubs.find(id))
                    .set(expiry_date.eq(&(chrono::offset::Utc::now() + chrono::Duration::days(3))))
                    .get_result::<Club>(conn);
                
                if let Ok(update) = update{
                    let member = ClubMember{
                        id: -1,
                        user_id: user_id,
                        club_id: update.id,
                        is_moderator: if is_head {"head".to_owned()} else {"true".to_owned()}
                    };
                    Ok(Json(ClubDetails::from_join((member, update), user_id, &conn).unwrap()))
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

#[put("/clubs/<id>/join")]
pub async fn join(user: User, db: Db, id: i32) -> std::result::Result<Json<ClubDetails>, status::Custom<Option<Json<JsonError>>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members};

    let user_id=user.id.clone();
    match user.get_membership_status_async(&db, &id).await {
        MembershipStatus::Unassociated => {
            let result = db.run(move |conn| {
                let club_exists = clubs.find(id).get_result::<Club>(conn);
                
                if club_exists.is_ok() {
                    let member = NewClubMember{
                        user_id: &user_id,
                        club_id: &id,
                        is_moderator: &"false",
                    };

                    let result = insert_into(club_members).values(member).get_result(conn);
                    Ok(Json(ClubDetails::from_join((result.unwrap(), club_exists.unwrap()), user_id, &conn).unwrap()))
                }else{
                    Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "The club you are trying to join does not exist.".to_owned()}))))
                }
            }).await;
            result
        },
        _ => {
            Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "User is already a member or moderator.".to_owned()}))))
        }
    }
}

#[put("/clubs/<id>/leave")]
pub async fn leave(user: User, db: Db, id: i32) -> std::result::Result<status::Accepted<()>, status::Custom<Option<Json<JsonError>>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, club_id, user_id};

    let user_id_copy = user.id.clone();
    match user.get_membership_status_async(&db, &id).await {
        MembershipStatus::Moderator(is_head) => {
            let result = db.run(move |conn| {
                let club_exists = clubs.find(id).get_result::<Club>(conn);
                
                if club_exists.is_ok() {
                    if !is_head{
                        let _result = diesel::delete(club_members).filter(club_id.eq(id)).filter(user_id.eq(&user_id_copy)).execute(conn).unwrap();
                        Ok(status::Accepted(None))
                    }else{
                        Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "You are the appointed head of the club appoint a new one or delete the club".to_owned()}))))
                    }
                }else{
                    Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "The club you are trying to leave does not exist.".to_owned()}))))
                }
            }).await;
            result
        },
        MembershipStatus::Member => {
            let result = db.run(move |conn| {
                let club_exists = clubs.find(id).get_result::<Club>(conn);
                
                if club_exists.is_ok() {
                    let _result = diesel::delete(club_members).filter(club_id.eq(id)).filter(user_id.eq(&user_id_copy)).execute(conn).unwrap();
                    Ok(status::Accepted(None))
                }else{
                    Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "The club you are trying to leave does not exist.".to_owned()}))))
                }
            }).await;
            result
        },
        _ => {
            Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "User is already unassociated with the club.".to_owned()}))))
        }
    }
}

#[derive(Deserialize)]
pub struct AppointModeratorRequestDTO {
    pub user_id: i32,
    pub appoint_to_head: bool,
}

#[put("/clubs/<id>/appoint", data = "<request>")]
pub async fn appoint(user: User, db: Db, id: i32, request: Json<AppointModeratorRequestDTO>) -> std::result::Result<status::Accepted<Json<ClubDetails>>, status::Custom<Option<Json<JsonError>>>> {
    use crate::schema::clubs::dsl::{clubs};
    use crate::schema::club_members::dsl::{club_members, club_id, user_id, is_moderator};

    let user_id_copy = user.id.clone();
    match user.get_membership_status_async(&db, &id).await {
        MembershipStatus::Moderator(is_head) => {
            let result = db.run(move |conn| {
                let club_exists = clubs.find(id).get_result::<Club>(conn);        
                if club_exists.is_ok() {
                    if is_head{
                        if let Some(fetched_user) = User::get_by_id(conn, &request.user_id){
                            match fetched_user.get_membership_status(conn, &request.user_id){
                                MembershipStatus::Moderator(fetched_user_is_head) => {
                                    if request.appoint_to_head == true && fetched_user_is_head == false{
                                        //Make current user just a moderator.
                                        let _res = diesel::update(club_members)
                                            .filter(club_id.eq(id))
                                            .filter(user_id.eq(&user_id_copy))
                                            .set(is_moderator.eq("true"))
                                            .execute(conn);
                                        //Appoint new user to head moderator.
                                        let _res = diesel::update(club_members)
                                            .filter(club_id.eq(id))
                                            .filter(user_id.eq(&request.user_id))
                                            .set(is_moderator.eq("head"))
                                            .execute(conn);
                                    } else {
                                        return Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "You already are a head moderator.".to_owned()}))))
                                    }
                                },
                                MembershipStatus::Member => {
                                    if request.appoint_to_head == true {
                                        //Make current user just a moderator.
                                        let _res = diesel::update(club_members)
                                            .filter(club_id.eq(id))
                                            .filter(user_id.eq(&user_id_copy))
                                            .set(is_moderator.eq("true"))
                                            .execute(conn);
                                        //Appoint new user to head moderator.
                                        let _res = diesel::update(club_members)
                                            .filter(club_id.eq(id))
                                            .filter(user_id.eq(&request.user_id))
                                            .set(is_moderator.eq("head"))
                                            .execute(conn);
                                    }else{
                                        //Appoint new user to moderator.
                                        let _res = diesel::update(club_members)
                                            .filter(club_id.eq(id))
                                            .filter(user_id.eq(&request.user_id))
                                            .set(is_moderator.eq("true"))
                                            .execute(conn);
                                    }
                                },
                                _ => {
                                    return Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "User is not a member.".to_owned()}))))
                                }
                            }
                            let club = clubs.find(id).get_result::<Club>(conn).unwrap();
                            Ok(status::Accepted(Some(Json(club.to_club_details(&conn, &user_id_copy)))))
                        }else{
                            Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "User does not exist.".to_owned()}))))
                        }
                    }else{
                        Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "Only the head moderator can appoint new moderators.".to_owned()}))))
                    }
                }else{
                    Err(status::Custom(Status::BadRequest, Some(Json(JsonError {error: "The club you are trying to leave does not exist.".to_owned()}))))
                }
            }).await;
            result
        },
        _ => {
            Err(status::Custom(Status::Forbidden, Some(Json(JsonError {error: "User is not a head moderator for this club.".to_owned()}))))
        }
    }
}

#[put("/clubs/<id>/logo", format = "image/png", data = "<file>")]
pub async fn upload(user: User, db: Db, id: i32, mut file: Capped<TempFile<'_>>) -> std::result::Result<status::Accepted<()>, status::Custom<Option<Json<JsonError>>>> {
    println!("MAde it this far");
    match user.get_membership_status_async(&db, &id).await {
        MembershipStatus::Moderator(is_head) => {
            if is_head{
                if file.is_complete() {
                    match file.move_copy_to(format!("uploads/{}.png",id)).await {
                        Ok(_) => {return Ok(status::Accepted(None))},
                        Err(e) => {println!("Error encountered while trying to persist a file, {:?}", e); return Err(status::Custom(Status::BadRequest, None))}
                    }
                } else {
                    return Err(status::Custom(Status::BadRequest, None))
                }
            }else {
                return Err(status::Custom(Status::Forbidden, None))
            }
        },
        _ => {
            return Err(status::Custom(Status::Forbidden, None))
        }
    }
}