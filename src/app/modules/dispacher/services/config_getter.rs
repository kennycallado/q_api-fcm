use serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ConfigGetter {
    pub fcm_api_key: Option<String>,
}

impl ConfigGetter {
    pub fn get_fcm_api_key() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .fcm_api_key
    }
}
