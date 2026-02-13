use super::acolytes::AcolyteState;
use super::generators::GeneratorState;
use bevy::prelude::*;

/// Three secondary resources that create strategic tension
#[derive(Resource, Debug)]
pub struct SecondaryResources {
    /// Patience-gated resource needed for high-tier generators
    pub serenity: f64,
    /// Click-generated resource that powers Moments of Clarity
    pub curiosity: f64,
    /// Depletable boost resource
    pub focus: f64,
    pub focus_max: f64,
    pub focus_active: bool,
    pub focus_drain_rate: f64,
    pub focus_regen_rate: f64,
    pub focus_multiplier: f64,
}

impl Default for SecondaryResources {
    fn default() -> Self {
        Self {
            serenity: 0.0,
            curiosity: 0.0,
            focus: 0.0,
            focus_max: 100.0,
            focus_active: false,
            focus_drain_rate: 2.0,
            focus_regen_rate: 0.1,
            focus_multiplier: 1.5,
        }
    }
}

impl SecondaryResources {
    pub fn focus_mult(&self) -> f32 {
        if self.focus_active {
            self.focus_multiplier as f32
        } else {
            1.0
        }
    }

    pub fn focus_mult_f64(&self) -> f64 {
        if self.focus_active {
            self.focus_multiplier
        } else {
            1.0
        }
    }
}

/// Serenity accumulates passively from acolytes and generators
pub fn generate_serenity(
    mut resources: ResMut<SecondaryResources>,
    acolytes: Res<AcolyteState>,
    generators: Res<GeneratorState>,
    time: Res<Time>,
) {
    let base_rate = 0.01;
    let acolyte_bonus = 0.005 * acolytes.count as f64;
    let generator_bonus = 0.001 * generators.owned.iter().sum::<u32>() as f64;
    let rate = base_rate + acolyte_bonus + generator_bonus;
    resources.serenity += rate * time.delta_secs() as f64;
}

/// Focus regens while inactive, drains while active. [G] to toggle.
pub fn update_focus(
    mut resources: ResMut<SecondaryResources>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs() as f64;

    if keys.just_pressed(KeyCode::KeyG) && !resources.focus_active && resources.focus >= 10.0 {
        resources.focus_active = true;
    }

    if resources.focus_active {
        resources.focus -= resources.focus_drain_rate * dt;
        if resources.focus <= 0.0 {
            resources.focus = 0.0;
            resources.focus_active = false;
        }
    } else {
        let max = resources.focus_max;
        resources.focus = (resources.focus + resources.focus_regen_rate * dt).min(max);
    }
}
