use crate::prelude::*;

#[derive(FromForm)]
pub struct GoogleTokenForm<'r> {
    pub credential: &'r str,
    pub g_csrf_token: &'r str,
}

#[post("/auth/login", data = "<token>")]
pub async fn login(token: Form<GoogleTokenForm<'_>>, cookies: &CookieJar<'_>) -> Redirect {
    let cred = token.credential.clone();
    let cookies_g_csrf_token = cookies.get_pending("g_crsf_token").map(|c| Box::new(c.value().to_owned()));
    for c in cookies.iter() {
        println!("Name: {:?}, Value: {:?}", c.name(), c.value());
    }
    println!("Cookies: {}", cookies_g_csrf_token.clone().unwrap_or(Box::new(String::new())));
    println!("Token: {}", token.g_csrf_token);
    if cookies_g_csrf_token.unwrap_or(Box::new(String::new())).as_str() == token.g_csrf_token{
        let cookie = Cookie::new("user_jwt", cred.to_owned()); 
        cookies.add_private(cookie);
    }
    Redirect::to("/")
}