use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

use crate::app::modules::tokens::model::{FcmToken, NewFcmToken};
use crate::app::modules::tokens::services::repository as fcm_token_repository;

async fn helper(db: &Db, new_fcm_token: NewFcmToken) -> Result<FcmToken, ()> {
    match fcm_token_repository::add_fcm_token(&db, new_fcm_token).await {
        Ok(fcm_token) => Ok(fcm_token),
        Err(e) => {
            // Detectar si es único en ese caso quizá actualizar...
            println!("Error: post_create_admin; {}", e);
            Err(())
        }
    }
}

pub async fn post_create_admin(
    db: Db,
    _user: UserInClaims,
    fcm_token: NewFcmToken,
) -> Result<Json<FcmToken>, Status> {
    match helper(&db, fcm_token).await {
        Ok(fcm_token) => Ok(Json(fcm_token)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// Para administrar permisos es necesario conectar con users_api
