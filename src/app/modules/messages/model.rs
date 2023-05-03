use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::database::schema::fcm_messages;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct FcmMessage {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = fcm_messages)]
#[serde(crate = "rocket::serde")]
pub struct NewFcmMessage {
    pub title: String,
    pub body: String,
}

impl From<FcmMessage> for NewFcmMessage {
    fn from(fcm_message: FcmMessage) -> Self {
        NewFcmMessage {
            title: fcm_message.title,
            body: fcm_message.body,
        }
    }
}
