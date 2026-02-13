use super::circle_material::{CircleMaterial, CircleParams};
use crate::gameplay::pondering::PonderState;
use bevy::gltf::GltfAssetLabel;
use bevy::prelude::*;
use bevy::scene::SceneRoot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamiliarType {
    RoundCat,
}

#[derive(Component)]
pub struct Familiar {
    pub familiar_type: FamiliarType,
    pub attention_timer: f32,
    pub max_attention_time: f32,
    pub has_been_petted: bool,
}

#[derive(Resource)]
pub struct FamiliarSpawnTimer {
    pub timer: Timer,
}

impl Default for FamiliarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct FamiliarHighlight {
    pub owner: Entity,
}

#[derive(Message)]
pub struct FamiliarPetted {
    pub familiar_type: FamiliarType,
}

pub fn spawn_familiar_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<FamiliarSpawnTimer>,
    asset_server: Res<AssetServer>,
    existing: Query<&Familiar>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut circle_materials: ResMut<Assets<CircleMaterial>>,
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() && existing.iter().count() < 1 {
        let x = 0.6;
        let z = 0.0;

        let familiar_entity = commands
            .spawn((
                Transform::from_xyz(x, 1.30, z)
                    .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
                Familiar {
                    familiar_type: FamiliarType::RoundCat,
                    attention_timer: 0.0,
                    max_attention_time: 15.0,
                    has_been_petted: false,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    SceneRoot(
                        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/cat.glb")),
                    ),
                    Transform::from_scale(Vec3::splat(0.5)),
                ));
            })
            .id();

        // Procedural magic circle on the table under the cat
        commands.spawn((
            Mesh3d(meshes.add(Circle::new(0.5))),
            MeshMaterial3d(circle_materials.add(CircleMaterial {
                params: CircleParams {
                    time: 0.0,
                    intensity: 1.0,
                    _pad0: 0.0,
                    _pad1: 0.0,
                },
            })),
            Transform::from_xyz(x, 1.02, z)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            FamiliarHighlight {
                owner: familiar_entity,
            },
        ));

        // Point light to cast golden glow on surfaces
        commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.8, 0.3),
                intensity: 15000.0,
                range: 3.0,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_xyz(x, 1.20, z),
            FamiliarHighlight {
                owner: familiar_entity,
            },
        ));
    }
}

pub fn familiar_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut familiars: Query<(Entity, &mut Familiar)>,
    highlights: Query<(Entity, &FamiliarHighlight)>,
    mut highlight_lights: Query<(&mut PointLight, &FamiliarHighlight)>,
    circle_query: Query<&MeshMaterial3d<CircleMaterial>, With<FamiliarHighlight>>,
    mut circle_materials: ResMut<Assets<CircleMaterial>>,
) {
    let t = time.elapsed_secs();

    for (entity, mut familiar) in &mut familiars {
        if !familiar.has_been_petted {
            familiar.attention_timer += time.delta_secs();

            // Neglected familiar wanders away
            if familiar.attention_timer >= familiar.max_attention_time {
                for (h_entity, highlight) in &highlights {
                    if highlight.owner == entity {
                        commands.entity(h_entity).despawn();
                    }
                }
                commands.entity(entity).despawn();
                return;
            }
        }
    }

    let pulse = (t * 2.0).sin() * 0.5 + 0.5;

    // Update magic circle shader time
    for mat_handle in &circle_query {
        if let Some(mat) = circle_materials.get_mut(mat_handle) {
            mat.params.time = t;
            mat.params.intensity = 0.8 + 0.4 * pulse;
        }
    }

    for (mut light, _) in &mut highlight_lights {
        light.intensity = 8000.0 + 12000.0 * pulse;
    }
}

pub fn handle_pet_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut familiars: Query<(&mut Familiar, Entity)>,
    highlights: Query<(Entity, &FamiliarHighlight)>,
    mut pet_messages: MessageWriter<FamiliarPetted>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        for (mut familiar, entity) in &mut familiars {
            if !familiar.has_been_petted {
                familiar.has_been_petted = true;
                pet_messages.write(FamiliarPetted {
                    familiar_type: familiar.familiar_type,
                });
                // Remove the highlight glow (cat stays on the table)
                for (h_entity, highlight) in &highlights {
                    if highlight.owner == entity {
                        commands.entity(h_entity).despawn();
                    }
                }
                break;
            }
        }
    }
}

pub fn apply_familiar_effects(
    mut ponder: ResMut<PonderState>,
    mut pet_messages: MessageReader<FamiliarPetted>,
) {
    for event in pet_messages.read() {
        match event.familiar_type {
            FamiliarType::RoundCat => {
                ponder.ponder_intensity = (ponder.ponder_intensity + 0.3).min(1.0);
            }
        }
    }
}
