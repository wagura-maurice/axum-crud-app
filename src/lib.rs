// src/lib.rs
use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new().route("/", get(root))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}

using SQLx i want an auth logi that will use d1 staorage an will have queri runnin to it to achive the flooing api requrment for the todo app

Auth: - hav a jwt logic similar to larvels santuc logi for api auth
    /sign-up
    /sign-in
    /request-otp
    /verify-otp/ inclus the signiture an token logic the the post request should have
    /forgot-password
    /reset-password/inclus the signiture an token logic the the post request should have
    /verify-account/inclus the signiture an token logic the the post request should have
    /sign-out/ this shoul be done using the bere token in the herder. thus it confirmas done or not you understant.

Application:
    /dashboard -show the home statiscis i.e return the json nedd to build up the reporting logic for the systams ui side of things
    /profile -show the home statiscis i.e return the json nedd to build up the reporting logic for the systams ui side of things
    /account -show the home statiscis i.e return the json nedd to build up the reporting logic for the systams ui side of things
    /settings -show the home statiscis i.e return the json nedd to build up the reporting logic for the systams ui side of things