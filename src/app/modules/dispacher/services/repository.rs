use diesel::prelude::*;

use crate::database::connection::Db;
use crate::database::schema::{fcm_messages, fcm_tokens};

use crate::app::modules::messages::model::{FcmMessage, NewFcmMessage};
use crate::app::modules::tokens::model::{FcmToken, NewFcmToken};

pub async fn get_message_by_id(db: &Db, id: i32) -> Result<FcmMessage, diesel::result::Error> {
    let message = db
        .run(move |conn| fcm_messages::table.find(id).first::<FcmMessage>(conn))
        .await;

    message
}

pub async fn get_tokens_by_user_ids(
    db: &Db,
    ids: Vec<i32>,
) -> Result<Vec<FcmToken>, diesel::result::Error> {
    let tokens = db
        .run(move |conn| {
            fcm_tokens::table
                .filter(fcm_tokens::user_id.eq_any(ids))
                .load::<FcmToken>(conn)
        })
        .await;

    tokens
}
