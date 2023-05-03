use std::borrow::Cow;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::database::schema::fcm_tokens;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[diesel(treat_none_as_null = true)]
#[serde(crate = "rocket::serde")]
pub struct FcmToken {
    pub id: i32,
    pub user_id: i32,
    pub token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = fcm_tokens)]
#[serde(crate = "rocket::serde")]
pub struct NewFcmToken {
    pub user_id: i32,
    pub token: Option<String>,
}

impl From<FcmToken> for NewFcmToken {
    fn from(fcm_token: FcmToken) -> Self {
        NewFcmToken {
            user_id: fcm_token.user_id,
            token: fcm_token.token,
        }
    }
}

impl Into<Cow<'_, str>> for FcmToken {
    fn into(self) -> Cow<'static, str> {
        match self.token {
            Some(token) => token.into(),
            None => "".into(),
        }
    }
}

impl AsRef<str> for FcmToken {
    fn as_ref(&self) -> &str {
        match &self.token {
            Some(token) => token.as_str(),
            None => "",
        }
    }
}
