use bevy::tasks::Task;
use std::{env, ops::Deref};
use surf::{self, HttpClient};

pub struct HttpResource(surf::Client);

impl Deref for HttpResource {
    type Target = surf::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for HttpResource {
    fn default() -> Self {
        let endpoint = surf::Url::parse(&env::var("ENDPOINT").expect("ENDPOINT not set"))
            .expect("Could not parse endpoint");
        let api_key = env::var("API_KEY").expect("API_KEY not set");
        let client: surf::Client = surf::Config::new()
            .set_base_url(endpoint)
            .add_header("apikey", api_key)
            .expect("headers wonky")
            .try_into()
            .unwrap();
        Self(client)
    }
}
