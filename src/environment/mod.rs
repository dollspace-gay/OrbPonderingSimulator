use bevy::prelude::*;

pub mod daynight;
pub mod lighting;
pub mod sky;
pub mod tower;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<sky::SkyMaterial>::default())
            .init_resource::<daynight::DayNightCycle>()
            .add_systems(
                Startup,
                (tower::spawn_tower, lighting::setup_lighting, sky::spawn_sky),
            )
            .add_systems(
                Update,
                (daynight::update_cycle, lighting::update_ambient_from_cycle),
            );
    }
}
