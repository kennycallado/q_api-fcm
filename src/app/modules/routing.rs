use crate::app::modules::dispacher::controller::routes as fcm_dispacher_routes;
use crate::app::modules::messages::controller::routes as fcm_messages_routes;
use crate::app::modules::tokens::controller::routes as fcm_tokens_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket
            .mount("/api/v1/fcm/token", fcm_tokens_routes())
            .mount("/api/v1/fcm/message", fcm_messages_routes())
            .mount("/api/v1/fcm/dispatch", fcm_dispacher_routes())
    })
}
