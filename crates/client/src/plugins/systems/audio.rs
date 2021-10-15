use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::PlaySfxEvent;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySfxEvent>().add_system(play_sfx_system);
    }
}

fn play_sfx_system(mut events: EventReader<PlaySfxEvent>, audio: Res<Audio>) {
    for event in events.iter() {
        event.play(&audio);
    }
}
