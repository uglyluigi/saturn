use crate::prelude::*;

#[derive(FromForm)]
pub struct GoogleTokenForm<'r> {
    pub credential: &'r str,
    pub g_csrf_token: &'r str,
}

#[post("/", data = "<token>")]
pub async fn login(token: Form<GoogleTokenForm<'_>>, cookies: &CookieJar<'_>) -> Result<String> {
    let cred = token.credential.clone();
    let cookie = Cookie::new("user_jwt", cred.to_owned()); 
    cookies.add_private(cookie);


    Ok(String::from("Cookie added"))
}