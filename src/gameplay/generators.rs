use super::moments::MomentState;
use super::schools::SchoolState;
use super::shop::PurchaseTracker;
use super::synergies::SynergyState;
use super::transcendence::TranscendenceState;
use super::wisdom::WisdomMeter;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratorType {
    Candle,
    CrystalBall,
    AncientTome,
    LeyLineTap,
    AstralMirror,
    DreamLoom,
    VoidGate,
    CosmicEye,
}

impl GeneratorType {
    pub const ALL: [GeneratorType; 8] = [
        GeneratorType::Candle,
        GeneratorType::CrystalBall,
        GeneratorType::AncientTome,
        GeneratorType::LeyLineTap,
        GeneratorType::AstralMirror,
        GeneratorType::DreamLoom,
        GeneratorType::VoidGate,
        GeneratorType::CosmicEye,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Candle => "Enchanted Candle",
            Self::CrystalBall => "Crystal Ball",
            Self::AncientTome => "Ancient Tome",
            Self::LeyLineTap => "Ley Line Tap",
            Self::AstralMirror => "Astral Mirror",
            Self::DreamLoom => "Dream Loom",
            Self::VoidGate => "Void Gate",
            Self::CosmicEye => "Cosmic Eye",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Candle => "A flickering flame that whispers forgotten truths.",
            Self::CrystalBall => "Gazes into the probable and the improbable alike.",
            Self::AncientTome => "Pages filled with wisdom that rewrites itself nightly.",
            Self::LeyLineTap => "Channels the ambient arcane energy flowing beneath the tower.",
            Self::AstralMirror => "Reflects thoughts from other planes of consciousness.",
            Self::DreamLoom => "Weaves subconscious threads into tangible insight.",
            Self::VoidGate => "A controlled aperture into the space between spaces.",
            Self::CosmicEye => "Perceives the universal pattern underlying all wisdom.",
        }
    }

    pub fn base_cost(&self) -> u64 {
        match self {
            Self::Candle => 50,
            Self::CrystalBall => 500,
            Self::AncientTome => 5_000,
            Self::LeyLineTap => 50_000,
            Self::AstralMirror => 500_000,
            Self::DreamLoom => 5_000_000,
            Self::VoidGate => 50_000_000,
            Self::CosmicEye => 500_000_000,
        }
    }

    pub fn base_production(&self) -> f64 {
        match self {
            Self::Candle => 0.1,
            Self::CrystalBall => 1.0,
            Self::AncientTome => 8.0,
            Self::LeyLineTap => 47.0,
            Self::AstralMirror => 260.0,
            Self::DreamLoom => 1_400.0,
            Self::VoidGate => 7_800.0,
            Self::CosmicEye => 44_000.0,
        }
    }

    pub fn cost_growth(&self) -> f64 {
        1.15
    }

    /// Cost of the next unit given current count, with optional discount (0.1 = 10% off)
    pub fn next_cost_discounted(&self, owned: u32, discount: f64) -> u64 {
        let base = self.base_cost() as f64 * self.cost_growth().powi(owned as i32);
        (base * (1.0 - discount)).ceil().max(1.0) as u64
    }

    /// Total production from all owned units (before global multipliers)
    pub fn production(&self, owned: u32) -> f64 {
        self.base_production() * owned as f64
    }

    /// Minimum total truths required before this generator becomes visible in the shop
    pub fn unlock_threshold(&self) -> u32 {
        match self {
            Self::Candle => 0,
            Self::CrystalBall => 3,
            Self::AncientTome => 10,
            Self::LeyLineTap => 25,
            Self::AstralMirror => 50,
            Self::DreamLoom => 100,
            Self::VoidGate => 200,
            Self::CosmicEye => 400,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct GeneratorState {
    pub owned: [u32; 8],
}

impl GeneratorState {
    pub fn count(&self, gtype: GeneratorType) -> u32 {
        self.owned[gtype as usize]
    }

    pub fn add(&mut self, gtype: GeneratorType) {
        self.owned[gtype as usize] += 1;
    }

    /// Total base wisdom/sec from all generators (before global multipliers)
    pub fn total_base_production(&self) -> f64 {
        GeneratorType::ALL
            .iter()
            .enumerate()
            .map(|(i, gt)| gt.production(self.owned[i]))
            .sum()
    }
}

pub fn passive_generator_wisdom(
    generators: Res<GeneratorState>,
    synergies: Res<SynergyState>,
    tracker: Res<PurchaseTracker>,
    moments: Res<MomentState>,
    transcendence: Res<TranscendenceState>,
    school: Res<SchoolState>,
    mut wisdom: ResMut<WisdomMeter>,
    time: Res<Time>,
) {
    let base = synergies.total_synergized_production(&generators);
    if base <= 0.0 {
        return;
    }
    let rate = base
        * (1.0 + tracker.efficiency_bonus as f64)
        * tracker.wisdom_speed_bonus as f64
        * moments.wisdom_multiplier() as f64
        * transcendence.passive_multiplier() as f64
        * school.passive_multiplier() as f64;
    wisdom.current += (rate * time.delta_secs() as f64) as f32;
}
