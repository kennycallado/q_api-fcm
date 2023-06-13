use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::tokens::model::FcmToken;
use crate::app::modules::tokens::services::repository as fcm_token_repository;

pub async fn get_show_admin(
    db: Db,
    _user: UserInClaims,
    id: i32,
) -> Result<Json<FcmToken>, Status> {
    let token = fcm_token_repository::get_fcm_token_by_id(&db, id).await;

    match token {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn get_show_by_user_admin(
    db: Db,
    _user: UserInClaims,
    id: i32,
) -> Result<Json<FcmToken>, Status> {
    let token = fcm_token_repository::get_fcm_token_by_user_id(&db, id).await;

    match token {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn get_show_by_user_user(
    db: Db,
    user: UserInClaims,
    id: i32,
) -> Result<Json<FcmToken>, Status> {
    if user.id != id {
        return Err(Status::Unauthorized);
    }

    let token = fcm_token_repository::get_fcm_token_by_user_id(&db, id).await;

    match token {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(Status::InternalServerError),
    }
}
