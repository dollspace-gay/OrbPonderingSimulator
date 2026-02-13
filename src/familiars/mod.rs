use bevy::prelude::*;

pub mod circle_material;
pub mod familiar;

pub struct FamiliarsPlugin;

impl Plugin for FamiliarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<circle_material::CircleMaterial>::default())
            .init_resource::<familiar::FamiliarSpawnTimer>()
            .add_message::<familiar::FamiliarPetted>()
            .add_systems(
                Update,
                (
                    familiar::spawn_familiar_timer,
                    familiar::familiar_movement,
                    familiar::handle_pet_input,
                    familiar::apply_familiar_effects,
                ),
            );
    }
}
