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
    pub exp: Option<usize>,
    pub id: Option<i32>,
    pub email: Option<String>,
    pub picture: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[get("/auth/details", rank=2)]
pub async fn details_admin(admin: Admin, key_state: &State<GoogleKeysState>, cookies: &CookieJar<'_>) -> Result<Json<AuthDetails>> {
    let mut exp = None;

    //Fetch the jwt token from the user's computer and set our validation algorithm.
    let jwt = cookies.get_private("user_jwt").map(|cookie| Box::new(cookie.value().to_owned()));
    let validation = Validation::new(Algorithm::RS256);
    
    //Load each key and check the signature and claims.
    for key in key_state.fetch_keys().await{
        if let Some(jwt) = jwt.clone(){
            match decode::<GoogleClaims>(&jwt, &DecodingKey::from_rsa_components(&key.n, &key.e), &validation) {
                Ok(c) => {exp=Some(c.claims.exp)},
                Err(e) => {println!("{:?}", e)}
            };
        }

        if exp.is_some() {break}
    }
    
    Ok(Json(AuthDetails{
        auth_level: AuthLevel::User,
        exp: exp,
        id: Some(admin.0.id),
        email: Some(admin.0.email),
        picture: Some(admin.0.picture),
        first_name: Some(admin.0.first_name),
        last_name: Some(admin.0.last_name)
    }))
}

#[get("/auth/details", rank=3)]
pub async fn details_user(user: User, key_state: &State<GoogleKeysState>, cookies: &CookieJar<'_>) -> Result<Json<AuthDetails>> {
    let mut exp = None;

    //Fetch the jwt token from the user's computer and set our validation algorithm.
    let jwt = cookies.get_private("user_jwt").map(|cookie| Box::new(cookie.value().to_owned()));
    let validation = Validation::new(Algorithm::RS256);
    
    //Load each key and check the signature and claims.
    for key in key_state.fetch_keys().await{
        if let Some(jwt) = jwt.clone(){
            match decode::<GoogleClaims>(&jwt, &DecodingKey::from_rsa_components(&key.n, &key.e), &validation) {
                Ok(c) => {exp=Some(c.claims.exp)},
                Err(e) => {println!("{:?}", e)}
            };
        }

        if exp.is_some() {break}
    }
    
    Ok(Json(AuthDetails{
        auth_level: AuthLevel::User,
        exp: exp,
        id: Some(user.id),
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