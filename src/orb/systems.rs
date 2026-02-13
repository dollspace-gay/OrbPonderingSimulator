use super::{
    material::{OrbMaterial, OrbParams},
    stand_material::{StandMaterial, StandParams},
    types::{EquippedOrb, Orb},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct OrbStand;

pub fn spawn_orb(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OrbMaterial>>,
    mut stand_materials: ResMut<Assets<StandMaterial>>,
) {
    let orb_mesh = Sphere::new(0.35).mesh().ico(7).unwrap();

    // Stand: short cylinder on the table, orb sits on top
    // Stand height=0.06, table top Y=1.0, stand center Y=1.03, stand top Y=1.06
    // Orb raised so bottom rests on stand top: center at 1.06 + 0.35 = 1.41
    let stand_mesh = Cylinder::new(0.2, 0.03);

    commands.spawn((
        Mesh3d(meshes.add(stand_mesh)),
        MeshMaterial3d(stand_materials.add(StandMaterial {
            params: StandParams {
                time: 0.0,
                _pad0: 0.0,
                _pad1: 0.0,
                _pad2: 0.0,
            },
        })),
        Transform::from_xyz(0.0, 1.03, 0.0),
        OrbStand,
    ));

    commands.spawn((
        Mesh3d(meshes.add(orb_mesh)),
        MeshMaterial3d(materials.add(OrbMaterial {
            params: OrbParams {
                pondering_power: 0.0,
                color_phase: 0.0,
                glow_intensity: 0.3,
                orb_type_index: 0,
            },
        })),
        Transform::from_xyz(0.0, 1.39, 0.0),
        Orb::default(),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.6, 3.0).looking_at(Vec3::new(0.0, 1.3, 0.0), Vec3::Y),
    ));

    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.25, 0.22, 0.35),
        brightness: 400.0,
        ..default()
    });

    commands.spawn((
        PointLight {
            color: Color::srgb(0.5, 0.6, 0.95),
            intensity: 5000.0,
            range: 20.0,
            ..default()
        },
        Transform::from_xyz(0.0, 2.5, 0.0),
    ));
}

pub fn update_orb_uniforms(
    orb_query: Query<(&Orb, &MeshMaterial3d<OrbMaterial>)>,
    mut materials: ResMut<Assets<OrbMaterial>>,
    time: Res<Time>,
    equipped: Res<EquippedOrb>,
) {
    for (orb, material_handle) in &orb_query {
        if let Some(material) = materials.get_mut(material_handle) {
            material.params.pondering_power = orb.pondering_power;
            material.params.color_phase = orb.color_phase + time.elapsed_secs() * 0.1;
            material.params.glow_intensity = orb.glow_intensity;
            material.params.orb_type_index = equipped.0.to_index();
        }
    }
}

pub fn update_stand_uniforms(
    stand_query: Query<&MeshMaterial3d<StandMaterial>, With<OrbStand>>,
    mut materials: ResMut<Assets<StandMaterial>>,
    time: Res<Time>,
) {
    for material_handle in &stand_query {
        if let Some(material) = materials.get_mut(material_handle) {
            material.params.time = time.elapsed_secs();
        }
    }
}
