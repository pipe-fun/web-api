use rocket::http::{Cookies, Cookie, SameSite};

#[get("/logout")]
pub fn logout(mut cookies: Cookies<'_>) {
    let cookie = Cookie::build("token", "remove")
        .domain("127.0.0.1")
        .expires(time::now())
        .max_age(time::Duration::zero())
        .path("/")
        .same_site(SameSite::Strict)
        .http_only(true)
        .finish();
    cookies.add(cookie);
}