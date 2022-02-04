use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use futures_lite::{future, Future};
use serde_json::json;
use surf::Error;

use crate::prelude::*;
use bevy::{
    prelude::*,
    tasks::{IoTaskPool, Task},
};
use serde::{Deserialize, Serialize};

pub struct HttpPlugin;

impl Plugin for HttpPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<HttpEvent>()
            .init_resource::<HttpResource>()
            .init_resource::<HighScoresResource>()
            .add_system(process_http_events_system)
            .add_system(process_http_tasks_system)
            .add_system_set(
                SystemSet::on_enter(GameState::post_game()).with_system(update_high_scores_system),
            );
    }
}

pub enum HttpEvent {
    InsertScore(ScoreRecord),
    SetHighScores(Vec<ScoreRecord>),
    RequestHighScoresRefresh,
    Error(String),
}

// Auto-uploads high score if a name has already been input, and retrieves an updated copy of high scores
fn update_high_scores_system(
    mut database_events: EventWriter<HttpEvent>,
    state: Res<State<GameState>>,
    name: Option<Res<UsernameResource>>,
) {
    if let GameState::PostGame(details) = state.current() {
        if let GameType::Endless(_) = details.game_type {
            if let Some(name) = name {
                database_events.send(HttpEvent::InsertScore(ScoreRecord {
                    score: details.score,
                    name: name.to_string(),
                }));
            }
            // TODO: Update high scores after sending
        }
    }
}

#[derive(Component, Clone)]
pub struct TaskComponent(pub Task<HttpEvent>);

fn process_http_tasks_system(
    mut cmd: Commands,
    mut events: EventWriter<HttpEvent>,
    mut tasks: Query<(Entity, &mut TaskComponent)>,
) {
    tasks.for_each_mut(|(entity, mut task)| {
        if let Some(ev) = future::block_on(future::poll_once(&mut *task.0)) {
            events.send(ev);

            // Task is complete, so despawn entity
            cmd.entity(entity).despawn_recursive();
        }
    });
}

fn process_http_events_system(
    mut cmd: Commands,
    mut events: EventReader<HttpEvent>,
    thread_pool: Res<IoTaskPool>,
    http: Res<HttpResource>,
) {
    events.iter().for_each(|event| match event {
        HttpEvent::InsertScore(record) => {
            let req = http.post("scores").body(json!(record));

            let task = thread_pool.spawn(async move {
                match req.await {
                    Ok(res) => match res.status().is_success() {
                        true => {
                            info!("User score uploaded");
                            HttpEvent::RequestHighScoresRefresh
                        }
                        false => HttpEvent::Error(format!(
                            "Error uploading user score with status: {}",
                            res.status(),
                        )),
                    },
                    Err(e) => {
                        HttpEvent::Error(format!("User score failed to upload with error {:?}", e))
                    }
                }
            });

            cmd.spawn().insert(TaskComponent(task));
        }
        HttpEvent::SetHighScores(scores) => {
            cmd.insert_resource(HighScoresResource(Some(scores.to_vec())));
        }
        HttpEvent::RequestHighScoresRefresh => {
            let qs: HashMap<&'static str, &'static str> =
                [("select", "*"), ("limit", "100")].into();
            let req = http
                .get("scores")
                .query(&qs)
                .expect("error serializing query params");

            let task = thread_pool.spawn(async move {
                match req.recv_json::<Vec<ScoreRecord>>().await {
                    Ok(scores) => HttpEvent::SetHighScores(scores),
                    Err(e) => HttpEvent::Error(e.to_string()),
                }
            });

            cmd.spawn().insert(TaskComponent(task));
        }
        HttpEvent::Error(e) => error!("{:?}", e),
    });
}
