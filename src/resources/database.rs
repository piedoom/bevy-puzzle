use bevy::tasks::Task;
use postgrest::Postgrest;
use std::env;

type DatabaseTask = Task<Result<reqwest::Response, reqwest::Error>>;
pub struct DatabaseResource {
    pub client: Postgrest,
    //pub insert_tasks: Vec<DatabaseTask>,
}

impl Default for DatabaseResource {
    fn default() -> Self {
        Self {
            client: Postgrest::new(env::var("ENDPOINT").expect("ENDPOINT not set"))
                .insert_header("apikey", env::var("API_KEY").expect("API_KEY not set")),
            // insert_tasks: Default::default(),
        }
    }
}
