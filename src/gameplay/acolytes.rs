use super::achievements::AchievementTracker;
use super::challenges::ChallengeState;
use super::moments::MomentState;
use super::progression::ArcaneProgress;
use super::resources::SecondaryResources;
use super::schools::SchoolState;
use super::shop::PurchaseTracker;
use super::transcendence::TranscendenceState;
use super::wisdom::WisdomMeter;
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct AcolyteState {
    pub count: u32,
    pub base_rate: f32,
    pub base_cost: u64,
    pub cost_growth: f32,
}

impl Default for AcolyteState {
    fn default() -> Self {
        Self {
            count: 0,
            base_rate: 0.2,
            base_cost: 20,
            cost_growth: 1.15,
        }
    }
}

impl AcolyteState {
    pub fn next_cost(&self) -> u64 {
        (self.base_cost as f64 * self.cost_growth.powi(self.count as i32) as f64).ceil() as u64
    }

    pub fn passive_rate(&self) -> f32 {
        self.count as f32 * self.base_rate
    }
}

pub fn summon_acolyte(
    keys: Res<ButtonInput<KeyCode>>,
    mut acolytes: ResMut<AcolyteState>,
    mut progress: ResMut<ArcaneProgress>,
) {
    if keys.just_pressed(KeyCode::KeyA) {
        let cost = acolytes.next_cost();
        if progress.focus_points >= cost {
            progress.focus_points -= cost;
            acolytes.count += 1;
        }
    }
}

pub fn passive_wisdom(
    acolytes: Res<AcolyteState>,
    tracker: Res<PurchaseTracker>,
    moments: Res<MomentState>,
    transcendence: Res<TranscendenceState>,
    school: Res<SchoolState>,
    achievements: Res<AchievementTracker>,
    challenges: Res<ChallengeState>,
    resources: Res<SecondaryResources>,
    mut wisdom: ResMut<WisdomMeter>,
    time: Res<Time>,
) {
    if acolytes.count == 0 {
        return;
    }
    let rate = acolytes.passive_rate()
        * (1.0 + tracker.efficiency_bonus)
        * tracker.wisdom_speed_bonus
        * moments.wisdom_multiplier()
        * transcendence.passive_multiplier()
        * school.passive_multiplier()
        * achievements.wisdom_multiplier()
        * challenges.passive_multiplier()
        * resources.focus_mult();
    wisdom.current += rate * time.delta_secs();
}
