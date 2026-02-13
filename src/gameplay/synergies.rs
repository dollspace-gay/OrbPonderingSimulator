use super::generators::{GeneratorState, GeneratorType};
use bevy::prelude::*;

/// A single synergy link: owning units of `source` boosts `target`'s production
struct SynergyLink {
    source: GeneratorType,
    target: GeneratorType,
    /// Bonus per source unit owned (e.g., 0.02 = +2% per unit)
    bonus_per_unit: f64,
}

/// Adjacent-tier synergies (+2% each way) and skip-tier synergies (+1% one way)
const SYNERGY_TABLE: &[SynergyLink] = &[
    // Adjacent pairs — each boosts the other at +2% per unit
    // Candle ↔ Crystal Ball: "Candlelight enhances scrying / Crystal visions kindle the flame"
    SynergyLink {
        source: GeneratorType::Candle,
        target: GeneratorType::CrystalBall,
        bonus_per_unit: 0.02,
    },
    SynergyLink {
        source: GeneratorType::CrystalBall,
        target: GeneratorType::Candle,
        bonus_per_unit: 0.02,
    },
    // Crystal Ball ↔ Ancient Tome: "Visions guide the reader / Written prophecies sharpen the lens"
    SynergyLink {
        source: GeneratorType::CrystalBall,
        target: GeneratorType::AncientTome,
        bonus_per_unit: 0.02,
    },
    SynergyLink {
        source: GeneratorType::AncientTome,
        target: GeneratorType::CrystalBall,
        bonus_per_unit: 0.02,
    },
    // Ancient Tome ↔ Ley Line Tap: "Arcane scripts channel ley energy / Ley currents animate the pages"
    SynergyLink {
        source: GeneratorType::AncientTome,
        target: GeneratorType::LeyLineTap,
        bonus_per_unit: 0.02,
    },
    SynergyLink {
        source: GeneratorType::LeyLineTap,
        target: GeneratorType::AncientTome,
        bonus_per_unit: 0.02,
    },
    // Ley Line Tap ↔ Astral Mirror: "Earth energy stabilizes reflections / Astral feedback amplifies the flow"
    SynergyLink {
        source: GeneratorType::LeyLineTap,
        target: GeneratorType::AstralMirror,
        bonus_per_unit: 0.02,
    },
    SynergyLink {
        source: GeneratorType::AstralMirror,
        target: GeneratorType::LeyLineTap,
        bonus_per_unit: 0.02,
    },
    // Astral Mirror ↔ Dream Loom: "Reflections weave into dreams / Dream threads polish the mirror"
    SynergyLink {
        source: GeneratorType::AstralMirror,
        target: GeneratorType::DreamLoom,
        bonus_per_unit: 0.02,
    },
    SynergyLink {
        source: GeneratorType::DreamLoom,
        target: GeneratorType::AstralMirror,
        bonus_per_unit: 0.02,
    },
    // Dream Loom ↔ Void Gate: "Dreams map the void / Void whispers inspire the loom"
    SynergyLink {
        source: GeneratorType::DreamLoom,
        target: GeneratorType::VoidGate,
        bonus_per_unit: 0.02,
    },
    SynergyLink {
        source: GeneratorType::VoidGate,
        target: GeneratorType::DreamLoom,
        bonus_per_unit: 0.02,
    },
    // Void Gate ↔ Cosmic Eye: "The void reveals cosmic truths / Cosmic sight widens the gate"
    SynergyLink {
        source: GeneratorType::VoidGate,
        target: GeneratorType::CosmicEye,
        bonus_per_unit: 0.02,
    },
    SynergyLink {
        source: GeneratorType::CosmicEye,
        target: GeneratorType::VoidGate,
        bonus_per_unit: 0.02,
    },
    // Skip-tier synergies (+1% one direction)
    // Candle → Ancient Tome: "Flame illuminates hidden passages"
    SynergyLink {
        source: GeneratorType::Candle,
        target: GeneratorType::AncientTome,
        bonus_per_unit: 0.01,
    },
    // Crystal Ball → Astral Mirror: "Scrying resonates with reflections"
    SynergyLink {
        source: GeneratorType::CrystalBall,
        target: GeneratorType::AstralMirror,
        bonus_per_unit: 0.01,
    },
    // Ley Line Tap → Void Gate: "Ley energy fuels the aperture"
    SynergyLink {
        source: GeneratorType::LeyLineTap,
        target: GeneratorType::VoidGate,
        bonus_per_unit: 0.01,
    },
    // Dream Loom → Cosmic Eye: "Dreams reveal the cosmic pattern"
    SynergyLink {
        source: GeneratorType::DreamLoom,
        target: GeneratorType::CosmicEye,
        bonus_per_unit: 0.01,
    },
];

/// Milestone thresholds: (owned count, production multiplier)
/// Player receives the highest milestone they've reached
const MILESTONES: [(u32, f64); 4] = [(5, 1.5), (10, 2.0), (25, 3.0), (50, 5.0)];

fn milestone_multiplier(owned: u32) -> f64 {
    let mut mult = 1.0;
    for &(threshold, m) in &MILESTONES {
        if owned >= threshold {
            mult = m;
        } else {
            break;
        }
    }
    mult
}

/// Cached per-generator multipliers from synergies and milestones
#[derive(Resource, Debug)]
pub struct SynergyState {
    /// 1.0 + sum of synergy bonuses for each generator
    pub synergy_mult: [f64; 8],
    /// Milestone multiplier for each generator
    pub milestone_mult: [f64; 8],
}

impl Default for SynergyState {
    fn default() -> Self {
        Self {
            synergy_mult: [1.0; 8],
            milestone_mult: [1.0; 8],
        }
    }
}

impl SynergyState {
    /// Combined synergy + milestone multiplier for a generator type
    pub fn total_mult(&self, gtype: GeneratorType) -> f64 {
        self.synergy_mult[gtype as usize] * self.milestone_mult[gtype as usize]
    }

    /// Total synergized production across all generators (before global multipliers)
    pub fn total_synergized_production(&self, generators: &GeneratorState) -> f64 {
        GeneratorType::ALL
            .iter()
            .enumerate()
            .map(|(i, gt)| {
                gt.base_production()
                    * generators.owned[i] as f64
                    * self.synergy_mult[i]
                    * self.milestone_mult[i]
            })
            .sum()
    }

    /// Get a human-readable summary of active synergy bonuses for a generator
    pub fn synergy_description(
        &self,
        gtype: GeneratorType,
        generators: &GeneratorState,
    ) -> Option<String> {
        let mut parts = Vec::new();

        for link in SYNERGY_TABLE {
            if link.target != gtype {
                continue;
            }
            let source_count = generators.count(link.source);
            if source_count == 0 {
                continue;
            }
            let bonus_pct = link.bonus_per_unit * source_count as f64 * 100.0;
            parts.push(format!("+{:.0}% from {}", bonus_pct, link.source.name()));
        }

        let idx = gtype as usize;
        if self.milestone_mult[idx] > 1.0 {
            parts.push(format!("x{:.1} milestone", self.milestone_mult[idx]));
        }

        if parts.is_empty() {
            None
        } else {
            Some(parts.join(", "))
        }
    }
}

/// Recalculates synergy and milestone multipliers when generator counts change
pub fn recalculate_synergies(generators: Res<GeneratorState>, mut synergies: ResMut<SynergyState>) {
    if !generators.is_changed() {
        return;
    }

    // Reset
    synergies.synergy_mult = [1.0; 8];
    synergies.milestone_mult = [1.0; 8];

    // Synergy bonuses
    for link in SYNERGY_TABLE {
        let source_count = generators.count(link.source);
        if source_count > 0 {
            let target_idx = link.target as usize;
            synergies.synergy_mult[target_idx] += link.bonus_per_unit * source_count as f64;
        }
    }

    // Milestone bonuses
    for (i, gt) in GeneratorType::ALL.iter().enumerate() {
        let owned = generators.count(*gt);
        synergies.milestone_mult[i] = milestone_multiplier(owned);
    }
}
