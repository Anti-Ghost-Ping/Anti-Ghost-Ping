use crate::structs::AgpContext;

pub mod database;
pub mod embed;
pub mod message;
pub mod api;

impl AgpContext {
    pub fn interaction(&self) -> twilight_http::client::InteractionClient {
        self.http.interaction(self.app_id)
    }
}