use super::shop::PurchaseTracker;
use super::wisdom::TruthGenerated;
use crate::orb::types::OrbType;
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct ArcaneProgress {
    pub focus_points: u64,
    pub total_truths: u32,
    pub unlocked_orbs: Vec<OrbType>,
}

impl Default for ArcaneProgress {
    fn default() -> Self {
        Self {
            focus_points: 0,
            total_truths: 0,
            unlocked_orbs: vec![OrbType::Crystal],
        }
    }
}

pub fn award_points(
    mut progress: ResMut<ArcaneProgress>,
    mut truth_messages: MessageReader<TruthGenerated>,
    tracker: Res<PurchaseTracker>,
) {
    for _msg in truth_messages.read() {
        progress.focus_points += 10 + tracker.afp_bonus as u64;
        progress.total_truths += 1;
    }
}
