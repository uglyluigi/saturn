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
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub profile_picture: Option<String>
}

#[get("/auth/details")]
pub async fn details_admin(admin: Admin) -> Result<Json<AuthDetails>> {
    Ok(Json(AuthDetails{
        auth_level: AuthLevel::Admin,
        email: Some(admin.0.email),
        ..Default::default()
    }))
}

#[get("/auth/details", rank=2)]
pub async fn details_user(user: User) -> Result<Json<AuthDetails>> {
    Ok(Json(AuthDetails{
        auth_level: AuthLevel::User,
        email: Some(user.email),
        ..Default::default()
    }))
}


#[get("/auth/details", rank=3)]
pub async fn details_guest() -> Result<Json<AuthDetails>> {
    
    Ok(Json(AuthDetails{
        auth_level: AuthLevel::Guest,
        ..Default::default()
    }))
}