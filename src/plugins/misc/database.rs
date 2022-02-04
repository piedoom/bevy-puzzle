use std::ops::{Deref, DerefMut};

use async_compat::Compat;
use futures_lite::future;

use crate::prelude::*;
use bevy::{prelude::*, tasks::IoTaskPool};
use serde::{Deserialize, Serialize};

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DatabaseEvent>()
            .init_resource::<DatabaseResource>()
            .init_resource::<HighScoresResource>()
            .add_system(process_database_events_system)
            .add_system_set(
                SystemSet::on_enter(GameState::post_game()).with_system(update_high_scores_system),
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

// Auto-uploads high score if a name has already been input, and retrieves an updated copy of high scores
fn update_high_scores_system(
    mut database_events: EventWriter<DatabaseEvent>,
    mut high_scores: ResMut<HighScoresResource>,
    database: Res<DatabaseResource>,
    thread_pool: Res<IoTaskPool>,
    state: Res<State<GameState>>,
    name: Option<Res<UsernameResource>>,
) {
    if let GameState::PostGame(details) = state.current() {
        if let GameType::Endless(_) = details.game_type {
            if let Some(name) = name {
                database_events.send(DatabaseEvent::InsertScore(ScoreRecord {
                    score: details.score,
                    name: name.to_string(),
                }));
            }
            // Update high scores after sending
            get_high_scores(&mut high_scores, &database, &thread_pool);
        }
    }
}

/// Retrieve and update a resource for high scores
fn get_high_scores(
    high_scores: &mut HighScoresResource,
    database: &DatabaseResource,
    thread_pool: &IoTaskPool,
) {
    let db = database
        .client
        .from("scores")
        .select("*")
        .limit(100)
        .execute();
    let mut task = thread_pool.spawn(Compat::new(async move { db.await }));
    match future::block_on(&mut task) {
        Ok(res) => {
            let mut json_task = thread_pool.spawn(Compat::new(async move {
                res.json::<Vec<ScoreRecord>>().await.ok()
            }));
            match future::block_on(&mut json_task) {
                Some(mut res) => {
                    **high_scores = {
                        // sort by score in inverse
                        res.sort_by(|a, b| b.score.cmp(&a.score));
                        Some(res)
                    }
                }
                None => (),
            }
        }
        Err(err) => bevy::log::error!("{}", format!("Database task error: {}", err).as_str()),
    }
}

fn process_database_events_system(
    mut events: EventReader<DatabaseEvent>,
    mut database: ResMut<DatabaseResource>,
    thread_pool: Res<IoTaskPool>,
) {
    events.iter().for_each(|event| match event {
        DatabaseEvent::InsertScore(record) => {
            let db = database
                .client
                .from("scores")
                .insert(serde_json::to_string(record).expect("Could not deserialize score"))
                .execute();

            match future::block_on(&mut thread_pool.spawn(Compat::new(async move { db.await }))) {
                Ok(res) => bevy::log::trace!("Successfully finished database task"),
                Err(err) => {
                    bevy::log::error!("{}", format!("Database task error: {}", err).as_str())
                }
            }
        }
    });
}

#[derive(Serialize, Deserialize)]
pub struct ScoreRecord {
    pub score: usize,
    pub name: String,
}
