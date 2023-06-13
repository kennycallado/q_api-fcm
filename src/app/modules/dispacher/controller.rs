use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::guards::claims::AccessClaims;

use crate::app::modules::messages::model::FcmMessage;
use crate::app::modules::tokens::model::FcmToken;

use crate::app::modules::dispacher::services::config_getter::ConfigGetter;
use crate::app::modules::dispacher::services::repository;

pub fn routes() -> Vec<rocket::Route> {
    routes![index_options, post_show_send,]
}

#[options("/")]
pub async fn index_options() -> Status {
    Status::Ok
}

#[post("/<message_id>", data = "<user_ids>", rank = 1)]
pub async fn post_show_send(
    db: Db,
    claims: AccessClaims,
    message_id: i32,
    user_ids: Json<Vec<i32>>,
) -> Result<Status, Status> {
    // Check the roles ???

    match claims.0.user.role.name.as_str() {
        "admin" => {
            let (message, tokens) = getter(&db, message_id, user_ids.into_inner()).await?;
            match sender(message, tokens).await {
                Ok(status) => return Ok(status),
                Err(status) => return Err(status),
            }
        }
        _ => {
            println!(
                "Error: get_show; Role not handled {}",
                claims.0.user.role.name
            );
            return Err(Status::BadRequest);
        }
    };
}

async fn sender(message: FcmMessage, tokens: Vec<FcmToken>) -> Result<Status, Status> {
    let client = fcm::Client::new();
    let api_key = match ConfigGetter::get_fcm_api_key() {
        Some(api_key) => api_key,
        None => {
            println!("Error: sender; There is no api key for FCM");
            return Err(Status::InternalServerError);
        }
    };

    let mut notification_builder = fcm::NotificationBuilder::new();
    notification_builder.title(message.title.as_str());
    notification_builder.body(message.body.as_str());
    let notification = notification_builder.finalize();

    let mut message_builder;
    if tokens.len() == 1 {
        message_builder = fcm::MessageBuilder::new(&api_key, tokens[0].as_ref());
    } else {
        message_builder = fcm::MessageBuilder::new_multi(&api_key, tokens.as_ref());
    }
    message_builder.notification(notification);

    let response = client.send(message_builder.finalize()).await;
    println!("Response: {:?}", response);

    match response {
        Ok(response) => {
            response.results.iter().for_each(|result| {
                println!("Result: {:?}", result);
            });
            Ok(Status::Ok)
        }
        Err(error) => {
            println!("Error: {:?}", error);
            Err(Status::InternalServerError)
        }
    }
}

async fn getter(
    db: &Db,
    id: i32,
    users: Vec<i32>,
) -> Result<(FcmMessage, Vec<FcmToken>), Status> {
    let message = repository::get_message_by_id(db, id).await;
    let tokens = repository::get_tokens_by_user_ids(db, users).await;

    match (message, tokens) {
        (Ok(message), Ok(tokens)) => Ok((message, tokens)),
        _ => Err(Status::InternalServerError),
    }
}
