use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrbType {
    Crystal,
    Obsidian,
    Mercury,
    Galaxy,
}

impl Default for OrbType {
    fn default() -> Self {
        Self::Crystal
    }
}

impl OrbType {
    pub fn to_index(&self) -> u32 {
        match self {
            OrbType::Crystal => 0,
            OrbType::Obsidian => 1,
            OrbType::Mercury => 2,
            OrbType::Galaxy => 3,
        }
    }
}

#[derive(Resource)]
pub struct EquippedOrb(pub OrbType);

impl Default for EquippedOrb {
    fn default() -> Self {
        Self(OrbType::Crystal)
    }
}

#[derive(Component)]
pub struct Orb {
    pub orb_type: OrbType,
    pub pondering_power: f32,
    pub color_phase: f32,
    pub glow_intensity: f32,
}

impl Default for Orb {
    fn default() -> Self {
        Self {
            orb_type: OrbType::Crystal,
            pondering_power: 0.0,
            color_phase: 0.0,
            glow_intensity: 0.3,
        }
    }
}
