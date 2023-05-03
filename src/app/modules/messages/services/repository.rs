use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::fcm_messages;

use crate::app::modules::messages::model::{FcmMessage, NewFcmMessage};

pub async fn get_all(db: &Db) -> Result<Vec<FcmMessage>, diesel::result::Error> {
    let messages = db
        .run(move |conn| fcm_messages::table.load::<FcmMessage>(conn))
        .await;

    messages
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<FcmMessage, diesel::result::Error> {
    let message = db
        .run(move |conn| fcm_messages::table.find(id).first::<FcmMessage>(conn))
        .await;

    message
}

pub async fn create(
    db: &Db,
    new_message: NewFcmMessage,
) -> Result<FcmMessage, diesel::result::Error> {
    let message = db
        .run(move |conn| {
            diesel::insert_into(fcm_messages::table)
                .values(&new_message)
                .get_result::<FcmMessage>(conn)
        })
        .await;

    message
}

pub async fn update(
    db: &Db,
    id: i32,
    new_message: NewFcmMessage,
) -> Result<FcmMessage, diesel::result::Error> {
    let message = db
        .run(move |conn| {
            diesel::update(fcm_messages::table.find(id))
                .set(&new_message)
                .get_result::<FcmMessage>(conn)
        })
        .await;

    message
}
