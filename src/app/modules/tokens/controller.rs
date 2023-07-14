use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::guards::claims::AccessClaims;

use crate::app::modules::tokens::handlers::{create, index, show, update};
use crate::app::modules::tokens::model::{FcmToken, NewFcmToken};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_all,
        get_index,
        get_index_none,
        get_show,
        get_show_none,
        get_show_by_user,
        get_show_by_user_none,
        post_create,
        post_create_none,
        put_update,
        put_update_none,
        put_update_by_user,
        put_update_by_user_none,
    ]
}

#[options("/<_..>")]
pub async fn options_all() -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: Db, claims: AccessClaims) -> Result<Json<Vec<FcmToken>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_admin(db, claims.0.user).await,
        _ => {
            println!(
                "Error: get_index; Role not handled :{}",
                claims.0.user.role.name
            );
            return Err(Status::BadRequest);
        }
    }
}

#[get("/", rank = 2)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 1)]
pub async fn get_show(db: Db, claims: AccessClaims, id: i32) -> Result<Json<FcmToken>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(db, claims.0.user, id).await,
        _ => {
            println!(
                "Error: get_show; Role not handled :{}",
                claims.0.user.role.name
            );
            return Err(Status::BadRequest);
        }
    }
}

#[get("/<_id>", rank = 2)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[get("/<id>/user", rank = 1)]
pub async fn get_show_by_user(
    db: Db,
    claims: AccessClaims,
    id: i32,
) -> Result<Json<FcmToken>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_by_user_admin(db, claims.0.user, id).await,
        "coord" => show::get_show_by_user_user(db, claims.0.user, id).await,
        "thera" => show::get_show_by_user_user(db, claims.0.user, id).await,
        "user" => show::get_show_by_user_user(db, claims.0.user, id).await,
        _ => {
            println!(
                "Error: get_show_by_user; Role not handled :{}",
                claims.0.user.role.name
            );
            return Err(Status::BadRequest);
        }
    }
}

#[get("/<_id>/user", rank = 2)]
pub async fn get_show_by_user_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_fcm_token>", rank = 1)]
pub async fn post_create(
    db: Db,
    claims: AccessClaims,
    new_fcm_token: Json<NewFcmToken>,
) -> Result<Json<FcmToken>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => {
            create::post_create_admin(db, claims.0.user, new_fcm_token.into_inner()).await
        }
        "robot" => {
            create::post_create_admin(db, claims.0.user, new_fcm_token.into_inner()).await
        }
        _ => {
            println!(
                "Error: post_create; Role not handled :{}",
                claims.0.user.role.name
            );
            return Err(Status::BadRequest);
        }
    }
}

#[post("/", rank = 2)]
pub async fn post_create_none() -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<new_fcm_token>", rank = 1)]
pub async fn put_update(
    db: Db,
    claims: AccessClaims,
    id: i32,
    new_fcm_token: Json<NewFcmToken>,
) -> Result<Json<FcmToken>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => {
            update::put_update_admin(db, claims.0.user, id, new_fcm_token.into_inner()).await
        }
        "coord" => {
            update::put_update_user(db, claims.0.user, id, new_fcm_token.into_inner()).await
        }
        "thera" => {
            update::put_update_user(db, claims.0.user, id, new_fcm_token.into_inner()).await
        }
        "user" => {
            update::put_update_user(db, claims.0.user, id, new_fcm_token.into_inner()).await
        }
        _ => {
            println!(
                "Error: put_update; Role not handled :{}",
                claims.0.user.role.name
            );
            return Err(Status::BadRequest);
        }
    }
}

#[put("/<_id>", rank = 2)]
pub async fn put_update_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[put("/<user_id>/user", data = "<new_fcm_token>", rank = 1)]
pub async fn put_update_by_user(db: Db, claims: AccessClaims, user_id: i32, new_fcm_token: Json<NewFcmToken>) -> Result<Json<FcmToken>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::put_update_by_user_admin(db, claims.0.user, user_id, new_fcm_token.into_inner()).await,
        
        "robot" => {
            update::put_update_by_user_admin(db, claims.0.user, user_id, new_fcm_token.into_inner())
                .await
        }
        "coord" => {
            update::put_update_by_user_user(db, claims.0.user, user_id, new_fcm_token.into_inner())
                .await
        }
        "thera" => {
            update::put_update_by_user_user(db, claims.0.user, user_id, new_fcm_token.into_inner())
                .await
        }
        "user" => {
            update::put_update_by_user_user(db, claims.0.user, user_id, new_fcm_token.into_inner())
                .await
        }
        _ => {
            println!(
                "Error: put_update_by_user; Role not handled :{}",
                claims.0.user.role.name
            );
            return Err(Status::BadRequest);
        }
    }
}

#[put("/<_id>/user", rank = 2)]
pub async fn put_update_by_user_none(_id: i32) -> Status {
    Status::Unauthorized
}
