use diesel::prelude::*;

use crate::database::connection::Db;
use crate::database::schema::fcm_tokens;

use crate::app::modules::tokens::model::{FcmToken, NewFcmToken};

pub async fn get_all_fcm_tokens(db: &Db) -> Result<Vec<FcmToken>, diesel::result::Error> {
    let tokens = db
        .run(move |conn| fcm_tokens::table.load::<FcmToken>(conn))
        .await?;

    Ok(tokens)
}

pub async fn get_fcm_token_by_id(db: &Db, id: i32) -> Result<FcmToken, diesel::result::Error> {
    let token = db
        .run(move |conn| fcm_tokens::table.find(id).first::<FcmToken>(conn))
        .await?;

    Ok(token)
}

pub async fn get_fcm_token_by_user_id(
    db: &Db,
    user_id: i32,
) -> Result<FcmToken, diesel::result::Error> {
    let token = db
        .run(move |conn| {
            fcm_tokens::table
                .filter(fcm_tokens::user_id.eq(user_id))
                .first::<FcmToken>(conn)
        })
        .await?;

    Ok(token)
}

pub async fn add_fcm_token(
    db: &Db,
    new_fcm_token: NewFcmToken,
) -> Result<FcmToken, diesel::result::Error> {
    let token = db
        .run(move |conn| {
            diesel::insert_into(fcm_tokens::table)
                .values(new_fcm_token)
                .get_result::<FcmToken>(conn)
        })
        .await?;

    Ok(token)
}

pub async fn update_fcm_token(
    db: &Db,
    id: i32,
    new_fcm_token: NewFcmToken,
) -> Result<FcmToken, diesel::result::Error> {
    let token = db
        .run(move |conn| {
            diesel::update(fcm_tokens::table.find(id))
                .set(new_fcm_token)
                .get_result::<FcmToken>(conn)
        })
        .await?;

    Ok(token)
}

pub async fn update_fcm_token_by_user_id(db: &Db, user_id: i32, new_fcm_token: NewFcmToken) -> Result<FcmToken, diesel::result::Error> {
    let token = db
        .run(move |conn| {
            diesel::update(fcm_tokens::table.filter(fcm_tokens::user_id.eq(user_id)))
                .set(new_fcm_token)
                .get_result::<FcmToken>(conn)
        })
        .await?;

    Ok(token)
}
