use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
};

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct CircleParams {
    pub time: f32,
    pub intensity: f32,
    pub _pad0: f32,
    pub _pad1: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CircleMaterial {
    #[uniform(0)]
    pub params: CircleParams,
}

impl Material for CircleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/magic_circle.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}
