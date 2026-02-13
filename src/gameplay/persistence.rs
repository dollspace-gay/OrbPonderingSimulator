use super::achievements::{AchievementId, AchievementTracker};
use super::acolytes::AcolyteState;
use super::challenges::{ChallengeId, ChallengeState};
use super::generators::GeneratorState;
use super::progression::ArcaneProgress;
use super::resources::SecondaryResources;
use super::schools::{SchoolOfThought, SchoolState};
use super::shadow_thoughts::ShadowState;
use super::shop::{PurchaseTracker, ShopItemId};
use super::synergies::SynergyState;
use super::transcendence::{EnlightenmentId, TranscendenceState};
use super::wisdom::WisdomMeter;
use crate::orb::types::{EquippedOrb, OrbType};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

// ========== SAVE DATA ==========

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub timestamp: u64,

    // Wisdom
    pub wisdom_current: f32,
    pub wisdom_max: f32,
    pub truths_generated: u32,

    // Progression
    pub focus_points: u64,
    pub total_truths: u32,

    // Acolytes
    pub acolyte_count: u32,

    // Generators
    pub generators_owned: [u32; 8],

    // Shop
    pub purchased_items: Vec<ShopItemId>,
    pub equipped_orb: OrbType,

    // Transcendence (permanent)
    pub insight: u32,
    pub total_transcendences: u32,
    pub purchased_enlightenments: Vec<EnlightenmentId>,
    pub run_wisdom_accumulated: f64,

    // School
    pub school: SchoolOfThought,
    pub school_run_truths: u32,

    // Achievements (permanent)
    pub unlocked_achievements: Vec<AchievementId>,
    pub lifetime_truths: u32,

    // Per-run achievement tracking
    pub achievement_peak_afp: u64,
    pub achievement_deep_focus_uses: u32,
    pub achievement_run_elapsed: f32,
    pub achievement_run_truths: u32,

    // Shadow thoughts
    #[serde(default)]
    pub shadow_count: u32,
    #[serde(default)]
    pub shadow_stored_wisdom: f64,

    // Challenges (permanent)
    #[serde(default)]
    pub completed_challenges: Vec<ChallengeId>,

    // Secondary resources (per-run)
    #[serde(default)]
    pub serenity: f64,
    #[serde(default)]
    pub curiosity: f64,
    #[serde(default)]
    pub focus: f64,
}

impl SaveData {
    pub fn capture(
        wisdom: &WisdomMeter,
        progress: &ArcaneProgress,
        acolytes: &AcolyteState,
        generators: &GeneratorState,
        tracker: &PurchaseTracker,
        equipped: &EquippedOrb,
        transcendence: &TranscendenceState,
        school: &SchoolState,
        achievements: &AchievementTracker,
        shadows: &ShadowState,
        challenges: &ChallengeState,
        resources: &SecondaryResources,
    ) -> Self {
        Self {
            version: 1,
            timestamp: now_secs(),
            wisdom_current: wisdom.current,
            wisdom_max: wisdom.max_wisdom,
            truths_generated: wisdom.truths_generated,
            focus_points: progress.focus_points,
            total_truths: progress.total_truths,
            acolyte_count: acolytes.count,
            generators_owned: generators.owned,
            purchased_items: tracker.purchased.iter().copied().collect(),
            equipped_orb: equipped.0,
            insight: transcendence.insight,
            total_transcendences: transcendence.total_transcendences,
            purchased_enlightenments: transcendence.purchased_enlightenments.clone(),
            run_wisdom_accumulated: transcendence.run_wisdom_accumulated,
            school: school.active,
            school_run_truths: school.run_truths,
            unlocked_achievements: achievements.unlocked.clone(),
            lifetime_truths: achievements.lifetime_truths,
            achievement_peak_afp: achievements.peak_afp,
            achievement_deep_focus_uses: achievements.deep_focus_uses,
            achievement_run_elapsed: achievements.run_elapsed,
            achievement_run_truths: achievements.run_truths,
            shadow_count: shadows.count,
            shadow_stored_wisdom: shadows.stored_wisdom,
            completed_challenges: challenges.completed.clone(),
            serenity: resources.serenity,
            curiosity: resources.curiosity,
            focus: resources.focus,
        }
    }

    pub fn restore(
        &self,
        wisdom: &mut WisdomMeter,
        progress: &mut ArcaneProgress,
        acolytes: &mut AcolyteState,
        generators: &mut GeneratorState,
        tracker: &mut PurchaseTracker,
        equipped: &mut EquippedOrb,
        transcendence: &mut TranscendenceState,
        school: &mut SchoolState,
        achievements: &mut AchievementTracker,
        synergies: &mut SynergyState,
        shadows: &mut ShadowState,
        challenges: &mut ChallengeState,
        resources: &mut SecondaryResources,
    ) {
        wisdom.current = self.wisdom_current;
        wisdom.max_wisdom = self.wisdom_max;
        wisdom.truths_generated = self.truths_generated;

        progress.focus_points = self.focus_points;
        progress.total_truths = self.total_truths;

        acolytes.count = self.acolyte_count;

        generators.owned = self.generators_owned;

        // Restore shop purchases and recalculate bonuses
        tracker.purchased = self.purchased_items.iter().copied().collect::<HashSet<_>>();
        tracker.recalculate(self.equipped_orb);
        equipped.0 = self.equipped_orb;

        transcendence.insight = self.insight;
        transcendence.total_transcendences = self.total_transcendences;
        transcendence.purchased_enlightenments = self.purchased_enlightenments.clone();
        transcendence.run_wisdom_accumulated = self.run_wisdom_accumulated;

        school.active = self.school;
        school.run_truths = self.school_run_truths;

        achievements.unlocked = self.unlocked_achievements.clone();
        achievements.lifetime_truths = self.lifetime_truths;
        achievements.peak_afp = self.achievement_peak_afp;
        achievements.deep_focus_uses = self.achievement_deep_focus_uses;
        achievements.run_elapsed = self.achievement_run_elapsed;
        achievements.run_truths = self.achievement_run_truths;

        shadows.count = self.shadow_count;
        shadows.stored_wisdom = self.shadow_stored_wisdom;

        challenges.completed = self.completed_challenges.clone();
        challenges.active = None;

        resources.serenity = self.serenity;
        resources.curiosity = self.curiosity;
        resources.focus = self.focus;

        // Recalculate synergies from restored generator state
        synergies.recalculate(generators);
    }
}

// ========== FILE I/O ==========

fn save_path() -> PathBuf {
    let mut path = dirs_or_fallback();
    path.push("orb_pondering_save.json");
    path
}

fn dirs_or_fallback() -> PathBuf {
    if let Some(data_dir) = std::env::var_os("APPDATA") {
        let mut p = PathBuf::from(data_dir);
        p.push("OrbPonderingSimulator");
        let _ = std::fs::create_dir_all(&p);
        p
    } else {
        PathBuf::from(".")
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn save_to_disk(data: &SaveData) {
    let path = save_path();
    match serde_json::to_string_pretty(data) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&path, json) {
                warn!("Failed to write save file: {}", e);
            }
        }
        Err(e) => warn!("Failed to serialize save: {}", e),
    }
}

pub fn load_from_disk() -> Option<SaveData> {
    let path = save_path();
    let data = std::fs::read_to_string(&path).ok()?;
    match serde_json::from_str(&data) {
        Ok(save) => Some(save),
        Err(e) => {
            warn!("Failed to parse save file: {}", e);
            None
        }
    }
}

// ========== OFFLINE PROGRESSION ==========

/// Maximum offline time in seconds (12 hours)
const MAX_OFFLINE_SECS: f64 = 12.0 * 60.0 * 60.0;

/// Offline production rate (50% of normal)
const OFFLINE_RATE: f64 = 0.5;

pub struct OfflineGains {
    pub wisdom_gained: f32,
    pub truths_earned: u32,
    pub afp_earned: u64,
    pub elapsed_secs: u64,
}

pub fn calculate_offline_gains(save: &SaveData) -> Option<OfflineGains> {
    let now = now_secs();
    if now <= save.timestamp {
        return None;
    }

    let raw_elapsed = (now - save.timestamp) as f64;
    if raw_elapsed < 60.0 {
        // Less than 1 minute away — skip
        return None;
    }
    let elapsed = raw_elapsed.min(MAX_OFFLINE_SECS);

    // Calculate passive rate from generators + acolytes
    // We approximate the rate at save time without synergy recalculation
    let mut gen_base: f64 = 0.0;
    for (i, &count) in save.generators_owned.iter().enumerate() {
        if count > 0 {
            let base_prod = match i {
                0 => 0.1,
                1 => 1.0,
                2 => 8.0,
                3 => 47.0,
                4 => 260.0,
                5 => 1_400.0,
                6 => 7_800.0,
                7 => 44_000.0,
                _ => 0.0,
            };
            gen_base += base_prod * count as f64;
        }
    }

    let acolyte_rate = save.acolyte_count as f64 * 0.2;
    let total_passive = gen_base + acolyte_rate;
    if total_passive <= 0.0 {
        return None;
    }

    // Apply offline rate
    let wisdom_per_sec = total_passive * OFFLINE_RATE;
    let total_wisdom = wisdom_per_sec * elapsed;

    // Calculate how many truths this would generate
    let mut wisdom_acc = save.wisdom_current as f64;
    let mut max_wisdom = save.wisdom_max as f64;
    let mut truths = 0u32;
    let mut remaining = total_wisdom;

    while remaining > 0.0 {
        let needed = max_wisdom - wisdom_acc;
        if needed <= 0.0 || remaining < needed {
            wisdom_acc += remaining;
            remaining = 0.0;
        } else {
            remaining -= needed;
            wisdom_acc = 0.0;
            truths += 1;
            max_wisdom *= 1.1; // Default scaling
        }
        // Safety cap
        if truths > 1000 {
            break;
        }
    }

    let afp_earned = truths as u64 * 10;

    Some(OfflineGains {
        wisdom_gained: wisdom_acc as f32 - save.wisdom_current,
        truths_earned: truths,
        afp_earned,
        elapsed_secs: raw_elapsed.min(MAX_OFFLINE_SECS) as u64,
    })
}

// ========== RESOURCES ==========

#[derive(Resource)]
pub struct AutoSaveTimer(pub Timer);

impl Default for AutoSaveTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(30.0, TimerMode::Repeating))
    }
}

#[derive(Resource, Default)]
pub struct OfflineReport(pub Option<OfflineGains>);

// ========== SYSTEMS ==========

/// On startup, load save and calculate offline gains
pub fn load_game(
    mut wisdom: ResMut<WisdomMeter>,
    mut progress: ResMut<ArcaneProgress>,
    mut acolytes: ResMut<AcolyteState>,
    mut generators: ResMut<GeneratorState>,
    mut tracker: ResMut<PurchaseTracker>,
    mut equipped: ResMut<EquippedOrb>,
    mut transcendence: ResMut<TranscendenceState>,
    mut school: ResMut<SchoolState>,
    mut achievements: ResMut<AchievementTracker>,
    mut synergies: ResMut<SynergyState>,
    mut shadows: ResMut<ShadowState>,
    mut challenges: ResMut<ChallengeState>,
    mut resources: ResMut<SecondaryResources>,
    mut offline_report: ResMut<OfflineReport>,
) {
    let Some(save) = load_from_disk() else {
        return;
    };

    // Calculate offline gains before restoring
    let gains = calculate_offline_gains(&save);

    // Restore game state
    save.restore(
        &mut wisdom,
        &mut progress,
        &mut acolytes,
        &mut generators,
        &mut tracker,
        &mut equipped,
        &mut transcendence,
        &mut school,
        &mut achievements,
        &mut synergies,
        &mut shadows,
        &mut challenges,
        &mut resources,
    );

    // Apply offline gains
    if let Some(ref g) = gains {
        wisdom.current += g.wisdom_gained;
        // Process truths earned offline
        for _ in 0..g.truths_earned {
            wisdom.truths_generated += 1;
            progress.total_truths += 1;
            achievements.lifetime_truths += 1;
            achievements.run_truths += 1;
        }
        progress.focus_points += g.afp_earned;
        // Set wisdom to accumulated amount after truths
        if g.truths_earned > 0 {
            // Recalculate max_wisdom after truths
            let scaling = school.scaling_override().unwrap_or(tracker.scaling_factor);
            for _ in 0..g.truths_earned {
                wisdom.max_wisdom *= scaling;
            }
        }
    }

    offline_report.0 = gains;
}

/// Auto-save on a timer
pub fn auto_save(
    mut timer: ResMut<AutoSaveTimer>,
    time: Res<Time>,
    wisdom: Res<WisdomMeter>,
    progress: Res<ArcaneProgress>,
    acolytes: Res<AcolyteState>,
    generators: Res<GeneratorState>,
    tracker: Res<PurchaseTracker>,
    equipped: Res<EquippedOrb>,
    transcendence: Res<TranscendenceState>,
    school: Res<SchoolState>,
    achievements: Res<AchievementTracker>,
    shadows: Res<ShadowState>,
    challenges: Res<ChallengeState>,
    resources: Res<SecondaryResources>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    let data = SaveData::capture(
        &wisdom,
        &progress,
        &acolytes,
        &generators,
        &tracker,
        &equipped,
        &transcendence,
        &school,
        &achievements,
        &shadows,
        &challenges,
        &resources,
    );
    save_to_disk(&data);
}

/// Save when the app is about to exit
pub fn save_on_exit(
    mut exit_messages: MessageReader<AppExit>,
    wisdom: Res<WisdomMeter>,
    progress: Res<ArcaneProgress>,
    acolytes: Res<AcolyteState>,
    generators: Res<GeneratorState>,
    tracker: Res<PurchaseTracker>,
    equipped: Res<EquippedOrb>,
    transcendence: Res<TranscendenceState>,
    school: Res<SchoolState>,
    achievements: Res<AchievementTracker>,
    shadows: Res<ShadowState>,
    challenges: Res<ChallengeState>,
    resources: Res<SecondaryResources>,
) {
    if exit_messages.read().next().is_none() {
        return;
    }

    let data = SaveData::capture(
        &wisdom,
        &progress,
        &acolytes,
        &generators,
        &tracker,
        &equipped,
        &transcendence,
        &school,
        &achievements,
        &shadows,
        &challenges,
        &resources,
    );
    save_to_disk(&data);
}

// ========== WELCOME-BACK UI ==========

#[derive(Component)]
pub struct WelcomeBackPanel;

#[derive(Component)]
pub struct WelcomeBackDismiss;

/// Shows the welcome-back overlay if there are offline gains
pub fn show_welcome_back(
    mut commands: Commands,
    report: Res<OfflineReport>,
) {
    let Some(ref gains) = report.0 else {
        return;
    };

    let hours = gains.elapsed_secs / 3600;
    let minutes = (gains.elapsed_secs % 3600) / 60;
    let time_str = if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.05, 0.85)),
            WelcomeBackPanel,
        ))
        .with_children(|backdrop| {
            backdrop
                .spawn((
                    Node {
                        width: Val::Px(420.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(28.0)),
                        row_gap: Val::Px(12.0),
                        border_radius: BorderRadius::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.06, 0.04, 0.14, 0.95)),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Text::new("Welcome Back, Ponderer"),
                        TextFont { font_size: 26.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.7, 1.0)),
                    ));

                    panel.spawn((
                        Text::new(format!("The orb meditated for {} while you were away.", time_str)),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgba(0.7, 0.65, 0.8, 0.8)),
                    ));

                    // Divider
                    panel.spawn((
                        Node { width: Val::Percent(80.0), height: Val::Px(1.0), ..default() },
                        BackgroundColor(Color::srgba(0.7, 0.5, 1.0, 0.3)),
                    ));

                    // Gains
                    if gains.truths_earned > 0 {
                        panel.spawn((
                            Text::new(format!("+{} truths discovered", gains.truths_earned)),
                            TextFont { font_size: 18.0, ..default() },
                            TextColor(Color::srgb(0.6, 0.9, 1.0)),
                        ));
                    }

                    if gains.afp_earned > 0 {
                        panel.spawn((
                            Text::new(format!("+{} Arcane Focus", gains.afp_earned)),
                            TextFont { font_size: 18.0, ..default() },
                            TextColor(Color::srgb(1.0, 0.85, 0.3)),
                        ));
                    }

                    if gains.wisdom_gained > 0.1 {
                        panel.spawn((
                            Text::new(format!("+{:.1} wisdom accumulated", gains.wisdom_gained)),
                            TextFont { font_size: 16.0, ..default() },
                            TextColor(Color::srgb(0.7, 0.5, 1.0)),
                        ));
                    }

                    panel.spawn((
                        Text::new("(Offline production: 50% rate, max 12 hours)"),
                        TextFont { font_size: 12.0, ..default() },
                        TextColor(Color::srgba(0.5, 0.5, 0.6, 0.5)),
                    ));

                    // Dismiss button
                    panel
                        .spawn((
                            Button,
                            Node {
                                padding: UiRect::axes(Val::Px(28.0), Val::Px(10.0)),
                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                margin: UiRect::top(Val::Px(8.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.7, 0.5, 1.0, 0.8)),
                            WelcomeBackDismiss,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("Continue Pondering"),
                                TextFont { font_size: 16.0, ..default() },
                                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                            ));
                        });
                });
        });
}

/// Handles dismissing the welcome-back panel
pub fn handle_welcome_dismiss(
    interactions: Query<&Interaction, (Changed<Interaction>, With<WelcomeBackDismiss>)>,
    mut commands: Commands,
    panels: Query<Entity, With<WelcomeBackPanel>>,
    mut report: ResMut<OfflineReport>,
) {
    for interaction in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        for entity in &panels {
            commands.entity(entity).despawn();
        }
        report.0 = None;
    }
}

/// Auto-dismiss welcome-back if user clicks anywhere or presses a key
pub fn auto_dismiss_welcome(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    panels: Query<Entity, With<WelcomeBackPanel>>,
    mut report: ResMut<OfflineReport>,
) {
    if report.0.is_none() {
        return;
    }

    if keys.get_just_pressed().len() > 0 {
        for entity in &panels {
            commands.entity(entity).despawn();
        }
        report.0 = None;
    }
}
