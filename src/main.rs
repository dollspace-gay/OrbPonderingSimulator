use bevy::prelude::*;

mod audio;
mod environment;
mod familiars;
mod gameplay;
mod orb;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Orb Pondering Simulator".into(),
                    resolution: (1280u32, 720u32).into(),
                    ..default()
                }),
                ..default()
            }),
            orb::OrbPlugin,
            gameplay::GameplayPlugin,
            environment::EnvironmentPlugin,
            familiars::FamiliarsPlugin,
            ui::UiPlugin,
            audio::GameAudioPlugin,
        ))
        .run();
}
