use crate::prelude::*;

#[post("/auth/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(Cookie::named("user_jwt"));
    Redirect::to("/")
}