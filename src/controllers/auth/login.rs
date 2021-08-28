use crate::prelude::*;

#[derive(FromForm)]
pub struct GoogleTokenForm<'r> {
    pub credential: &'r str,
    pub g_csrf_token: &'r str,
}

/*
This endpoint is designed to receive a POST form
field request from Google's oauth process. It 
should contain a jwt token and g_crsf_token. We
will store the former in cookies assuming the 
latter matches its copy which was sent to us via
cookies.
*/
#[post("/auth/login", data = "<token>")]
pub async fn login(token: Form<GoogleTokenForm<'_>>, cookies: &CookieJar<'_>) -> Redirect {
    let cred = token.credential.clone();
    let mut cookies_g_csrf_token = String::new();
    for c in cookies.iter() {
        if c.name() == "g_csrf_token"{
            cookies_g_csrf_token=c.value().to_owned();
        }
    }
    if cookies_g_csrf_token == token.g_csrf_token{
        let cookie = Cookie::new("user_jwt", cred.to_owned()); 
        cookies.add_private(cookie);
    }
    Redirect::to("/")
}