use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub enum AuthLevel{
    Admin,
    User,
    Guest
}

impl Default for AuthLevel {
    fn default() -> Self { AuthLevel::Guest }
}

#[derive(Serialize, Deserialize, Default)]
pub struct AuthDetails{
    pub auth_level: AuthLevel,
    pub email: Option<String>,
    pub picture: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[get("/auth/details", rank=2)]
pub async fn details_admin(admin: Admin) -> Result<Json<AuthDetails>> {
    Ok(Json(AuthDetails{
        auth_level: AuthLevel::User,
        email: Some(admin.0.email),
        picture: Some(admin.0.picture),
        first_name: Some(admin.0.first_name),
        last_name: Some(admin.0.last_name)
    }))
}

#[get("/auth/details", rank=3)]
pub async fn details_user(user: User) -> Result<Json<AuthDetails>> {
    Ok(Json(AuthDetails{
        auth_level: AuthLevel::User,
        email: Some(user.email),
        picture: Some(user.picture),
        first_name: Some(user.first_name),
        last_name: Some(user.last_name)
    }))
}


/*
If the User guard fails on a registered path it will return a 403.
This code below is here to catch that failure and return details_guest
assuming the route is correct. Otherwise it will just say the user is
not authorized.
*/

#[catch(403)]
pub async fn forbidden_or_details_guest(req: &Request<'_>) -> std::result::Result<status::Custom<Json<AuthDetails>>, status::Forbidden<Json<JsonError>>> {
    if req.uri().path() == "/api/auth/details" {
        //details_guest
        Ok(status::Custom(Status::Ok, Json(AuthDetails{
            auth_level: AuthLevel::Guest,
            ..Default::default()
        })))
    } else{
        Err(status::Forbidden(Some(Json(JsonError{error: "Client not authorized to perform that action.".to_owned()}))))
    }
}