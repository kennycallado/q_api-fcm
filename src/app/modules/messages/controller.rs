use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::guards::claims::AccessClaims;

use crate::app::modules::messages::model::FcmMessage;
use crate::app::modules::messages::services::repository as messages_repository;

use super::model::NewFcmMessage;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        index_options,
        show_options,
        get_index,
        get_index_none,
        get_show,
        get_show_none,
        post_create,
        post_create_none,
        put_update,
        put_update_none,
    ]
}

#[options("/")]
pub async fn index_options() -> Status {
    Status::Ok
}

#[options("/<_id>")]
pub async fn show_options(_id: i32) -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: Db, claims: AccessClaims) -> Result<Json<Vec<FcmMessage>>, Status> {
    let messages = match claims.0.user.role.name.as_str() {
        "admin" => messages_repository::get_all(&db).await,
        "coord" => messages_repository::get_all(&db).await,
        "thera" => messages_repository::get_all(&db).await,
        _ => {
            println!(
                "Error: get_index; Role not handled {}",
                claims.0.user.role.name
            );
            return Err(Status::BadRequest);
        }
    };

    match messages {
        Ok(messages) => Ok(Json(messages)),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/", rank = 2)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 1)]
pub async fn get_show(
    db: Db,
    claims: AccessClaims,
    id: i32,
) -> Result<Json<FcmMessage>, Status> {
    let message = match claims.0.user.role.name.as_str() {
        "admin" => messages_repository::get_by_id(&db, id).await,
        "coord" => messages_repository::get_by_id(&db, id).await,
        "thera" => messages_repository::get_by_id(&db, id).await,
        _ => {
            println!(
                "Error: get_show; Role not handled {}",
                claims.0.user.role.name
            );
            return Err(Status::BadRequest);
        }
    };

    match message {
        Ok(message) => Ok(Json(message)),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/<_id>", rank = 2)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_message>", rank = 1)]
pub async fn post_create(
    db: Db,
    claims: AccessClaims,
    new_message: Json<NewFcmMessage>,
) -> Result<Json<FcmMessage>, Status> {
    let message = match claims.0.user.role.name.as_str() {
        "admin" => messages_repository::create(&db, new_message.into_inner()).await,
        "coord" => messages_repository::create(&db, new_message.into_inner()).await,
        "thera" => messages_repository::create(&db, new_message.into_inner()).await,
        _ => {
            println!(
                "Error: post_create; Role not handled {}",
                claims.0.user.role.name
            );
            return Err(Status::BadRequest);
        }
    };

    match message {
        Ok(message) => Ok(Json(message)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/", data = "<_new_message>", rank = 2)]
pub async fn post_create_none(_new_message: Json<NewFcmMessage>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<new_message>", rank = 1)]
pub async fn put_update(
    db: Db,
    claims: AccessClaims,
    id: i32,
    new_message: Json<NewFcmMessage>,
) -> Result<Json<FcmMessage>, Status> {
    let message = match claims.0.user.role.name.as_str() {
        "admin" => messages_repository::update(&db, id, new_message.into_inner()).await,
        "coord" => messages_repository::update(&db, id, new_message.into_inner()).await,
        "thera" => messages_repository::update(&db, id, new_message.into_inner()).await,
        _ => {
            println!(
                "Error: put_update; Role not handled {}",
                claims.0.user.role.name
            );
            return Err(Status::BadRequest);
        }
    };

    match message {
        Ok(message) => Ok(Json(message)),
        Err(_) => Err(Status::NotFound),
    }
}

#[put("/<_id>", data = "<_new_message>", rank = 2)]
pub async fn put_update_none(_id: i32, _new_message: Json<NewFcmMessage>) -> Status {
    Status::Unauthorized
}
