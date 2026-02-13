use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
};

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct StandParams {
    pub time: f32,
    pub _pad0: f32,
    pub _pad1: f32,
    pub _pad2: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StandMaterial {
    #[uniform(0)]
    pub params: StandParams,
}

impl Material for StandMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/stand.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}
