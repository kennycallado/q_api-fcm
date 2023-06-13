use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::tokens::model::{FcmToken, NewFcmToken};
use crate::app::modules::tokens::services::repository as fcm_token_repository;

async fn helper(db: &Db, token_id: i32, new_fcm_token: NewFcmToken) -> Result<FcmToken, ()> {
    match fcm_token_repository::update_fcm_token(&db, token_id, new_fcm_token).await {
        Ok(fcm_token) => Ok(fcm_token),
        Err(_) => Err(()),
    }
}

async fn helper_by_user_id(db: &Db, user_id: i32, new_fcm_token: NewFcmToken) -> Result<FcmToken, ()> {
    match fcm_token_repository::update_fcm_token_by_user_id(&db, user_id, new_fcm_token).await {
        Ok(fcm_token) => Ok(fcm_token),
        Err(_) => Err(()),
    }
}

pub async fn put_update_admin(
    db: Db,
    _admin: UserInClaims,
    id: i32,
    new_fcm_token: NewFcmToken,
) -> Result<Json<FcmToken>, Status> {
    match helper(&db, id, new_fcm_token).await {
        Ok(fcm_token) => Ok(Json(fcm_token)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn put_update_user(
    db: Db,
    user: UserInClaims,
    id: i32,
    new_fcm_token: NewFcmToken,
) -> Result<Json<FcmToken>, Status> {
    if user.id != id {
        return Err(Status::Unauthorized);
    }

    match helper(&db, id, new_fcm_token).await {
        Ok(fcm_token) => Ok(Json(fcm_token)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn put_update_by_user_admin(db: Db, _admin: UserInClaims, user_id: i32, new_fcm_token: NewFcmToken) -> Result<Json<FcmToken>, Status> {
    match helper_by_user_id(&db, user_id, new_fcm_token).await {
        Ok(fcm_token) => Ok(Json(fcm_token)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn put_update_by_user_user(
    db: Db,
    user: UserInClaims,
    user_id: i32,
    new_fcm_token: NewFcmToken,
) -> Result<Json<FcmToken>, Status> {
    if user.id != user_id {
        return Err(Status::Unauthorized);
    }

    match helper_by_user_id(&db, user_id, new_fcm_token).await {
        Ok(fcm_token) => Ok(Json(fcm_token)),
        Err(_) => Err(Status::InternalServerError),
    }
}
