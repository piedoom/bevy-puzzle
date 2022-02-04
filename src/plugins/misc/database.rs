use std::ops::{Deref, DerefMut};

use async_compat::Compat;

use crate::prelude::*;
use bevy::{prelude::*, tasks::IoTaskPool};
use serde::{Deserialize, Serialize};

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DatabaseEvent>()
            .init_resource::<DatabaseResource>()
            .add_system(process_database_events_system)
            //.add_system(process_database_tasks_system)
            .add_system_set(
                SystemSet::on_enter(GameState::post_game()).with_system(upload_high_scores_system),
            );
    }
}

pub struct UsernameResource(String);

impl UsernameResource {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl Deref for UsernameResource {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UsernameResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub enum DatabaseEvent {
    InsertScore(ScoreRecord),
}

// Auto-uploads high score if a name has already been input
fn upload_high_scores_system(
    mut database_events: EventWriter<DatabaseEvent>,
    state: Res<State<GameState>>,
    name: Option<Res<UsernameResource>>,
) {
    if let GameState::PostGame(details) = state.current() {
        if let GameType::Endless(_) = details.game_type {
            if let Some(name) = name {
                database_events.send(DatabaseEvent::InsertScore(ScoreRecord {
                    score: details.score,
                    name: name.to_string(),
                }))
            }
        }
    }
}

fn process_database_events_system(
    mut events: EventReader<DatabaseEvent>,
    database: Res<DatabaseResource>,
    thread_pool: Res<IoTaskPool>,
) {
    events.iter().for_each(|event| match event {
        DatabaseEvent::InsertScore(record) => {
            let db = database
                .client
                .from("scores")
                .insert(serde_json::to_string(record).expect("Could not deserialize score"))
                .execute();

            thread_pool
                .spawn(Compat::new(async move { db.await }))
                .detach();
        }
    })
}

// fn process_database_tasks_system(mut database: ResMut<DatabaseResource>) {
//     database.tasks.iter_mut().for_each(|task| {
//         // Complete the database task
//         if let Some(result) = future::block_on(future::poll_once(&mut *task)) {
//             match result {
//                 Ok(_resp) => {
//                     bevy::log::trace!("Successfully finished database task");
//                 }
//                 Err(error) => {
//                     bevy::log::error!("{}", format!("Database task error: {}", error).as_str());
//                 }
//             }
//         }
//     });
// }

#[derive(Serialize, Deserialize)]
pub struct ScoreRecord {
    pub score: usize,
    pub name: String,
}
