use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SkyMaterial {
    #[uniform(0)]
    pub seed: f32,
}

impl Material for SkyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sky.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}

pub fn spawn_sky(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<SkyMaterial>>,
) {
    let sky_mesh = Sphere::new(500.0).mesh().ico(4).unwrap();

    // Negative scale flips normals so inside faces render (camera is inside the sphere)
    commands.spawn((
        Mesh3d(meshes.add(sky_mesh)),
        MeshMaterial3d(sky_materials.add(SkyMaterial { seed: 42.0 })),
        Transform::from_scale(Vec3::splat(-1.0)),
    ));
}
