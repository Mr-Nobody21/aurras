mod auth;
pub mod openwhisk_action;
pub mod user;
pub mod openwhisk_rule_trigger;

use crate::errors::AppError;
use actix_web::{web, web::ServiceConfig, HttpResponse};
use auth::*;
use openwhisk_action::*;
use openwhisk_rule_trigger::*;
use std::{fs::File, io::Read};
use user::*;

type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

// Handle configuration
pub fn app_config(config: &mut ServiceConfig) {
    let index = web::resource("/").route(web::get().to(index));
    let signup = web::resource("/signup").route(web::post().to(create_user));
    let me = web::resource("/whoami").route(web::get().to(user_info));
    let auth = web::resource("/auth").route(web::post().to(auth));

    let action_create = web::resource("/action")
        .route(web::post().to(action_create));

    let trigger_create = web::resource("/trigger")
        .route(web::post().to(create_trigger));
    
    let rule_create = web::resource("/rule")
        .route(web::post().to(create_rule));

    let delete = web::resource("/delete")
        .route(web::post().to(delete));

    let get_list = web::resource("/get_list").route(web::post().to(get_list));

    let update_rule = web::resource("/update_rule").route(web::post().to(update_rule));

    config
        .service(index)
        .service(signup)
        .service(me)
        .service(auth)
        .service(action_create)
        .service(trigger_create)
        .service(delete)
        .service(get_list)
        .service(rule_create)
        .service(update_rule);
}

// Index file handle
#[allow(unused_must_use)]
pub async fn index() -> HttpResponse {
    let mut f = File::open("src/html/index.html").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);
    HttpResponse::Ok().content_type("text/html").body(buffer)
}
