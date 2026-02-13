use super::achievements::AchievementTracker;
use super::acolytes::AcolyteState;
use super::challenges::ChallengeState;
use super::moments::MomentState;
use super::resources::SecondaryResources;
use super::schools::SchoolState;
use super::shop::PurchaseTracker;
use super::transcendence::TranscendenceState;
use super::wisdom::WisdomMeter;
use crate::orb::types::Orb;
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PonderState {
    pub ponder_intensity: f32,
    pub deep_focus_active: bool,
    pub deep_focus_timer: f32,
    pub deep_focus_cooldown: f32,
}

impl Default for PonderState {
    fn default() -> Self {
        Self {
            ponder_intensity: 0.0,
            deep_focus_active: false,
            deep_focus_timer: 0.0,
            deep_focus_cooldown: 0.0,
        }
    }
}

pub fn handle_click_ponder(
    mouse: Res<ButtonInput<MouseButton>>,
    mut wisdom: ResMut<WisdomMeter>,
    mut ponder: ResMut<PonderState>,
    tracker: Res<PurchaseTracker>,
    moments: Res<MomentState>,
    transcendence: Res<TranscendenceState>,
    school: Res<SchoolState>,
    achievements: Res<AchievementTracker>,
    challenges: Res<ChallengeState>,
    mut resources: ResMut<SecondaryResources>,
    interactions: Query<&Interaction>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    // Don't ponder if clicking a UI button
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            return;
        }
    }

    let deep_focus_mult = if ponder.deep_focus_active { 3.0 } else { 1.0 };
    let moment_click_mult = moments.click_multiplier();
    let enlightenment_mult = transcendence.click_multiplier();
    let school_click_mult = school.click_multiplier();
    let achievement_mult = achievements.wisdom_multiplier();
    let challenge_click_mult = challenges.click_multiplier();
    let gain = 1.0
        * (1.0 + tracker.efficiency_bonus)
        * tracker.wisdom_speed_bonus
        * deep_focus_mult
        * moment_click_mult
        * enlightenment_mult
        * school_click_mult
        * achievement_mult
        * challenge_click_mult
        * resources.focus_mult();

    wisdom.current += gain;
    ponder.ponder_intensity = 1.0;
    resources.curiosity += 1.0;
}

pub fn handle_deep_focus(keys: Res<ButtonInput<KeyCode>>, mut ponder: ResMut<PonderState>) {
    if keys.just_pressed(KeyCode::Space)
        && ponder.deep_focus_cooldown <= 0.0
        && !ponder.deep_focus_active
    {
        ponder.deep_focus_active = true;
        ponder.deep_focus_timer = 10.0;
        ponder.deep_focus_cooldown = 60.0;
    }
}

pub fn update_ponder_visuals(
    mut ponder: ResMut<PonderState>,
    acolytes: Res<AcolyteState>,
    time: Res<Time>,
    mut orb_query: Query<&mut Orb>,
) {
    let dt = time.delta_secs();

    // Tick Deep Focus timers
    if ponder.deep_focus_active {
        ponder.deep_focus_timer -= dt;
        if ponder.deep_focus_timer <= 0.0 {
            ponder.deep_focus_active = false;
            ponder.deep_focus_timer = 0.0;
        }
    }
    if ponder.deep_focus_cooldown > 0.0 {
        ponder.deep_focus_cooldown = (ponder.deep_focus_cooldown - dt).max(0.0);
    }

    // Base glow from acolytes
    let acolyte_glow = 0.1 + 0.03 * (acolytes.count.min(15) as f32);
    let base_level = if ponder.deep_focus_active {
        acolyte_glow.max(0.6)
    } else {
        acolyte_glow
    };

    // Decay intensity toward base level
    let decay_rate = 3.0;
    if ponder.ponder_intensity > base_level {
        ponder.ponder_intensity = (ponder.ponder_intensity - dt * decay_rate).max(base_level);
    } else {
        ponder.ponder_intensity = base_level;
    }

    // Update orb visuals
    for mut orb in &mut orb_query {
        orb.pondering_power = ponder.ponder_intensity;
        orb.glow_intensity = 0.3 + ponder.ponder_intensity * 0.7;
    }
}
