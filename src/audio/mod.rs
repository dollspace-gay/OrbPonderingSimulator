use bevy::prelude::*;

pub mod ambient;
pub mod reactive;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, _app: &mut App) {
        // Audio systems will be registered when audio assets are available.
    }
}
