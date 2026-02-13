use bevy::prelude::*;

pub mod material;
pub mod stand_material;
pub mod systems;
pub mod types;

pub struct OrbPlugin;

impl Plugin for OrbPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<material::OrbMaterial>::default())
            .add_plugins(MaterialPlugin::<stand_material::StandMaterial>::default())
            .init_resource::<types::EquippedOrb>()
            .add_systems(Startup, systems::spawn_orb)
            .add_systems(
                Update,
                (systems::update_orb_uniforms, systems::update_stand_uniforms),
            );
    }
}
