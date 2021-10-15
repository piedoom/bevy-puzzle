use bevy::prelude::{Handle, Res};
use bevy_kira_audio::{Audio, AudioSource};

pub struct PlaySfxEvent(Handle<AudioSource>);

impl PlaySfxEvent {
    pub fn new(handle: Handle<AudioSource>) -> Self {
        Self(handle)
    }
    pub fn play(&self, audio: &Res<Audio>) {
        audio.play(self.0.clone());
    }
}
