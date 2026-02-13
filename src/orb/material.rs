use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
};

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct OrbParams {
    pub pondering_power: f32,
    pub color_phase: f32,
    pub glow_intensity: f32,
    pub orb_type_index: u32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OrbMaterial {
    #[uniform(0)]
    pub params: OrbParams,
}

impl Material for OrbMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/orb.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
