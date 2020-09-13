use rocket::http::{Cookies, Cookie, SameSite};
use crate::request::COOKIE_DOMAIN;

#[get("/logout")]
pub fn logout(mut cookies: Cookies<'_>) {
    let cookie = Cookie::build("token", "remove")
        .domain(COOKIE_DOMAIN)
        .expires(time::now())
        .max_age(time::Duration::zero())
        .path("/")
        .same_site(SameSite::Strict)
        .http_only(true)
        .finish();
    cookies.add(cookie);
}