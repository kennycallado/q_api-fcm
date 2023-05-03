use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

use crate::app::modules::tokens::model::FcmToken;
use crate::app::modules::tokens::services::repository as fcm_token_repository;

pub async fn get_index_admin(
    db: Db,
    _user: UserInClaims,
) -> Result<Json<Vec<FcmToken>>, Status> {
    let tokens = fcm_token_repository::get_all_fcm_tokens(&db).await;

    match tokens {
        Ok(tokens) => Ok(Json(tokens)),
        Err(_) => Err(Status::NotFound),
    }
}
