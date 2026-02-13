use super::daynight::DayNightCycle;
use bevy::prelude::*;

pub fn setup_lighting(mut commands: Commands) {
    // Main moonlight from upper left
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            color: Color::srgb(0.6, 0.6, 0.8),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.3, 0.0)),
    ));

    // Fill light from the right (softer, warmer)
    commands.spawn((
        DirectionalLight {
            illuminance: 1500.0,
            color: Color::srgb(0.8, 0.6, 0.4),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.3, -0.8, 0.0)),
    ));
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn update_ambient_from_cycle(
    cycle: Res<DayNightCycle>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    let t = (cycle.time_of_day * std::f32::consts::TAU).sin() * 0.5 + 0.5;
    ambient.color = Color::srgb(lerp(0.05, 0.2, t), lerp(0.05, 0.2, t), lerp(0.15, 0.25, t));
    ambient.brightness = lerp(300.0, 500.0, t);
}
