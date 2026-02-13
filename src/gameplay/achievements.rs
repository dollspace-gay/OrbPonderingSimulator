use super::acolytes::AcolyteState;
use super::generators::{GeneratorState, GeneratorType};
use super::progression::ArcaneProgress;
use super::transcendence::TranscendenceState;
use super::wisdom::TruthGenerated;
use bevy::prelude::*;

// ========== ACHIEVEMENT DEFINITIONS ==========

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AchievementId {
    // Truth milestones
    FirstTruth,
    TenTruths,
    FiftyTruths,
    HundredTruths,
    FiveHundredTruths,
    ThousandTruths,

    // AFP milestones
    HundredAfp,
    ThousandAfp,
    HundredKAfp,
    MillionAfp,

    // Transcendence
    FirstTranscendence,
    FiveTranscendences,
    TenTranscendences,

    // Generators
    FirstGenerator,
    AllGeneratorTypes,
    FiftyCandles,
    HundredGenerators,

    // Acolytes
    FirstAcolyte,
    TenAcolytes,
    TwentyFiveAcolytes,

    // Hidden
    SpeedPonderer,
    DeepThinker,
    TruthSeeker,
}

impl AchievementId {
    pub const ALL: [AchievementId; 23] = [
        Self::FirstTruth,
        Self::TenTruths,
        Self::FiftyTruths,
        Self::HundredTruths,
        Self::FiveHundredTruths,
        Self::ThousandTruths,
        Self::HundredAfp,
        Self::ThousandAfp,
        Self::HundredKAfp,
        Self::MillionAfp,
        Self::FirstTranscendence,
        Self::FiveTranscendences,
        Self::TenTranscendences,
        Self::FirstGenerator,
        Self::AllGeneratorTypes,
        Self::FiftyCandles,
        Self::HundredGenerators,
        Self::FirstAcolyte,
        Self::TenAcolytes,
        Self::TwentyFiveAcolytes,
        Self::SpeedPonderer,
        Self::DeepThinker,
        Self::TruthSeeker,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::FirstTruth => "First Insight",
            Self::TenTruths => "Apprentice Ponderer",
            Self::FiftyTruths => "Seasoned Thinker",
            Self::HundredTruths => "Centurion of Wisdom",
            Self::FiveHundredTruths => "Sage of the Tower",
            Self::ThousandTruths => "Grand Philosopher",
            Self::HundredAfp => "Arcane Dabbler",
            Self::ThousandAfp => "Focus Adept",
            Self::HundredKAfp => "Arcane Reservoir",
            Self::MillionAfp => "Master of Focus",
            Self::FirstTranscendence => "Beyond the Veil",
            Self::FiveTranscendences => "Cycle Walker",
            Self::TenTranscendences => "Eternal Return",
            Self::FirstGenerator => "Automated Wisdom",
            Self::AllGeneratorTypes => "Full Arsenal",
            Self::FiftyCandles => "Candle Hoarder",
            Self::HundredGenerators => "Factory of Thought",
            Self::FirstAcolyte => "First Follower",
            Self::TenAcolytes => "Small Gathering",
            Self::TwentyFiveAcolytes => "Growing Order",
            Self::SpeedPonderer => "Swift Awakening",
            Self::DeepThinker => "Into the Deep",
            Self::TruthSeeker => "Collector of Oddities",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::FirstTruth => "Generate your first truth.",
            Self::TenTruths => "Generate 10 truths across all runs.",
            Self::FiftyTruths => "Generate 50 truths across all runs.",
            Self::HundredTruths => "Generate 100 truths across all runs.",
            Self::FiveHundredTruths => "Generate 500 truths across all runs.",
            Self::ThousandTruths => "Generate 1,000 truths across all runs.",
            Self::HundredAfp => "Accumulate 100 AFP in a single run.",
            Self::ThousandAfp => "Accumulate 1,000 AFP in a single run.",
            Self::HundredKAfp => "Accumulate 100,000 AFP in a single run.",
            Self::MillionAfp => "Accumulate 1,000,000 AFP in a single run.",
            Self::FirstTranscendence => "Transcend for the first time.",
            Self::FiveTranscendences => "Transcend 5 times.",
            Self::TenTranscendences => "Transcend 10 times.",
            Self::FirstGenerator => "Purchase your first generator.",
            Self::AllGeneratorTypes => "Own at least one of every generator type.",
            Self::FiftyCandles => "Own 50 Enchanted Candles.",
            Self::HundredGenerators => "Own 100 generators total.",
            Self::FirstAcolyte => "Summon your first acolyte.",
            Self::TenAcolytes => "Have 10 acolytes at once.",
            Self::TwentyFiveAcolytes => "Have 25 acolytes at once.",
            Self::SpeedPonderer => "Generate a truth within 30 seconds of starting a run.",
            Self::DeepThinker => "Use Deep Focus 10 times in a single run.",
            Self::TruthSeeker => "Generate 50 truths in a single run.",
        }
    }

    pub fn hidden_description(&self) -> &'static str {
        match self {
            Self::SpeedPonderer => "Speed is its own reward. ???",
            Self::DeepThinker => "Go deeper. ???",
            Self::TruthSeeker => "One run to rule them all. ???",
            _ => self.description(),
        }
    }

    pub fn is_hidden(&self) -> bool {
        matches!(
            self,
            Self::SpeedPonderer | Self::DeepThinker | Self::TruthSeeker
        )
    }

    /// Permanent wisdom multiplier bonus when unlocked (additive)
    pub fn reward_multiplier(&self) -> f32 {
        match self {
            Self::FirstTruth => 0.01,
            Self::TenTruths => 0.02,
            Self::FiftyTruths => 0.03,
            Self::HundredTruths => 0.05,
            Self::FiveHundredTruths => 0.08,
            Self::ThousandTruths => 0.12,
            Self::HundredAfp => 0.01,
            Self::ThousandAfp => 0.02,
            Self::HundredKAfp => 0.05,
            Self::MillionAfp => 0.10,
            Self::FirstTranscendence => 0.05,
            Self::FiveTranscendences => 0.08,
            Self::TenTranscendences => 0.12,
            Self::FirstGenerator => 0.01,
            Self::AllGeneratorTypes => 0.10,
            Self::FiftyCandles => 0.03,
            Self::HundredGenerators => 0.05,
            Self::FirstAcolyte => 0.01,
            Self::TenAcolytes => 0.03,
            Self::TwentyFiveAcolytes => 0.05,
            Self::SpeedPonderer => 0.05,
            Self::DeepThinker => 0.04,
            Self::TruthSeeker => 0.06,
        }
    }

    pub fn color(&self) -> Color {
        if self.is_hidden() {
            Color::srgb(1.0, 0.5, 0.3) // Orange for hidden/secret
        } else {
            match self.reward_multiplier() {
                r if r >= 0.10 => Color::srgb(1.0, 0.85, 0.3), // Gold for big rewards
                r if r >= 0.05 => Color::srgb(0.7, 0.5, 1.0),  // Purple for medium
                _ => Color::srgb(0.5, 0.8, 1.0),                // Blue for small
            }
        }
    }
}

// ========== TRACKER ==========

#[derive(Resource, Debug)]
pub struct AchievementTracker {
    pub unlocked: Vec<AchievementId>,
    /// Truths generated across all runs
    pub lifetime_truths: u32,
    /// Peak AFP reached in current run
    pub peak_afp: u64,
    /// Deep Focus uses this run
    pub deep_focus_uses: u32,
    /// Time elapsed this run (seconds)
    pub run_elapsed: f32,
    /// Truths in current run
    pub run_truths: u32,
    /// Queue of achievements to show notifications for
    pub notification_queue: Vec<AchievementId>,
}

impl Default for AchievementTracker {
    fn default() -> Self {
        Self {
            unlocked: Vec::new(),
            lifetime_truths: 0,
            peak_afp: 0,
            deep_focus_uses: 0,
            run_elapsed: 0.0,
            run_truths: 0,
            notification_queue: Vec::new(),
        }
    }
}

impl AchievementTracker {
    pub fn has(&self, id: AchievementId) -> bool {
        self.unlocked.contains(&id)
    }

    fn unlock(&mut self, id: AchievementId) {
        if !self.has(id) {
            self.unlocked.push(id);
            self.notification_queue.push(id);
        }
    }

    /// Total permanent wisdom multiplier from all unlocked achievements (1.0 = no bonus)
    pub fn wisdom_multiplier(&self) -> f32 {
        1.0 + self
            .unlocked
            .iter()
            .map(|a| a.reward_multiplier())
            .sum::<f32>()
    }

    /// Reset per-run tracking stats (called on transcendence)
    pub fn reset_run_stats(&mut self) {
        self.peak_afp = 0;
        self.deep_focus_uses = 0;
        self.run_elapsed = 0.0;
        self.run_truths = 0;
    }
}

// ========== SYSTEMS ==========

/// Tracks lifetime truths and run truths from TruthGenerated messages
pub fn track_achievement_stats(
    mut tracker: ResMut<AchievementTracker>,
    mut truth_messages: MessageReader<TruthGenerated>,
    progress: Res<ArcaneProgress>,
    time: Res<Time>,
) {
    let truth_count = truth_messages.read().count() as u32;
    if truth_count > 0 {
        tracker.lifetime_truths += truth_count;
        tracker.run_truths += truth_count;
    }

    // Track peak AFP
    if progress.focus_points > tracker.peak_afp {
        tracker.peak_afp = progress.focus_points;
    }

    // Track run time
    tracker.run_elapsed += time.delta_secs();
}

/// Tracks deep focus activations
pub fn track_deep_focus_uses(
    keys: Res<ButtonInput<KeyCode>>,
    ponder: Res<super::pondering::PonderState>,
    mut tracker: ResMut<AchievementTracker>,
) {
    // Detect when deep focus was just activated
    if keys.just_pressed(KeyCode::Space) && !ponder.deep_focus_active && ponder.deep_focus_cooldown <= 0.0 {
        tracker.deep_focus_uses += 1;
    }
}

/// Checks all achievement conditions and unlocks any that are met
pub fn check_achievements(
    mut tracker: ResMut<AchievementTracker>,
    generators: Res<GeneratorState>,
    acolytes: Res<AcolyteState>,
    transcendence: Res<TranscendenceState>,
) {
    // Truth milestones (lifetime)
    let lt = tracker.lifetime_truths;
    if lt >= 1 {
        tracker.unlock(AchievementId::FirstTruth);
    }
    if lt >= 10 {
        tracker.unlock(AchievementId::TenTruths);
    }
    if lt >= 50 {
        tracker.unlock(AchievementId::FiftyTruths);
    }
    if lt >= 100 {
        tracker.unlock(AchievementId::HundredTruths);
    }
    if lt >= 500 {
        tracker.unlock(AchievementId::FiveHundredTruths);
    }
    if lt >= 1000 {
        tracker.unlock(AchievementId::ThousandTruths);
    }

    // AFP milestones (peak in current run)
    let afp = tracker.peak_afp;
    if afp >= 100 {
        tracker.unlock(AchievementId::HundredAfp);
    }
    if afp >= 1_000 {
        tracker.unlock(AchievementId::ThousandAfp);
    }
    if afp >= 100_000 {
        tracker.unlock(AchievementId::HundredKAfp);
    }
    if afp >= 1_000_000 {
        tracker.unlock(AchievementId::MillionAfp);
    }

    // Transcendence
    let tc = transcendence.total_transcendences;
    if tc >= 1 {
        tracker.unlock(AchievementId::FirstTranscendence);
    }
    if tc >= 5 {
        tracker.unlock(AchievementId::FiveTranscendences);
    }
    if tc >= 10 {
        tracker.unlock(AchievementId::TenTranscendences);
    }

    // Generators
    let total_gens: u32 = generators.owned.iter().sum();
    if total_gens >= 1 {
        tracker.unlock(AchievementId::FirstGenerator);
    }
    if total_gens >= 100 {
        tracker.unlock(AchievementId::HundredGenerators);
    }

    let all_types = GeneratorType::ALL.iter().enumerate().all(|(i, _)| generators.owned[i] > 0);
    if all_types {
        tracker.unlock(AchievementId::AllGeneratorTypes);
    }

    if generators.count(GeneratorType::Candle) >= 50 {
        tracker.unlock(AchievementId::FiftyCandles);
    }

    // Acolytes
    if acolytes.count >= 1 {
        tracker.unlock(AchievementId::FirstAcolyte);
    }
    if acolytes.count >= 10 {
        tracker.unlock(AchievementId::TenAcolytes);
    }
    if acolytes.count >= 25 {
        tracker.unlock(AchievementId::TwentyFiveAcolytes);
    }

    // Hidden achievements
    // Speed Ponderer: truth within 30s of run start
    if tracker.run_truths >= 1 && tracker.run_elapsed <= 30.0 {
        tracker.unlock(AchievementId::SpeedPonderer);
    }

    // Deep Thinker: 10 deep focus uses in a run
    if tracker.deep_focus_uses >= 10 {
        tracker.unlock(AchievementId::DeepThinker);
    }

    // Truth Seeker: 50 truths in a single run
    if tracker.run_truths >= 50 {
        tracker.unlock(AchievementId::TruthSeeker);
    }
}

// ========== NOTIFICATION UI ==========

#[derive(Component)]
pub struct AchievementNotification {
    pub timer: Timer,
}

/// Spawns notification popups for newly unlocked achievements
pub fn spawn_notifications(
    mut commands: Commands,
    mut tracker: ResMut<AchievementTracker>,
) {
    while let Some(id) = tracker.notification_queue.pop() {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Percent(50.0),
                margin: UiRect::left(Val::Px(-175.0)),
                width: Val::Px(350.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(14.0)),
                row_gap: Val::Px(4.0),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.05, 0.15, 0.92)),
            AchievementNotification {
                timer: Timer::from_seconds(4.0, TimerMode::Once),
            },
        ))
        .with_children(|popup| {
            popup.spawn((
                Text::new("Achievement Unlocked!"),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::srgba(1.0, 0.85, 0.3, 0.8)),
            ));
            popup.spawn((
                Text::new(id.name()),
                TextFont { font_size: 20.0, ..default() },
                TextColor(id.color()),
            ));
            popup.spawn((
                Text::new(id.description()),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgba(0.8, 0.75, 0.85, 0.7)),
            ));
            popup.spawn((
                Text::new(format!("+{:.0}% wisdom", id.reward_multiplier() * 100.0)),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgba(0.5, 1.0, 0.5, 0.8)),
            ));
        });
    }
}

/// Ticks notification timers and despawns expired ones
pub fn update_notifications(
    mut commands: Commands,
    time: Res<Time>,
    mut notifications: Query<(Entity, &mut AchievementNotification)>,
) {
    for (entity, mut notif) in &mut notifications {
        notif.timer.tick(time.delta());
        if notif.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

// ========== ACHIEVEMENTS PANEL UI ==========

#[derive(Component)]
pub struct AchievementsPanel;

pub fn toggle_achievements(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<super::state::GameState>>,
    mut next_state: ResMut<NextState<super::state::GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyV) {
        match current_state.get() {
            super::state::GameState::Playing => {
                next_state.set(super::state::GameState::AchievementsOpen);
            }
            super::state::GameState::AchievementsOpen => {
                next_state.set(super::state::GameState::Playing);
            }
            _ => {}
        }
    }
}

pub fn open_achievements(mut commands: Commands, tracker: Res<AchievementTracker>) {
    let total_unlocked = tracker.unlocked.len();
    let total_achievements = AchievementId::ALL.len();
    let bonus = tracker.wisdom_multiplier();

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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            AchievementsPanel,
        ))
        .with_children(|backdrop| {
            backdrop
                .spawn((
                    Node {
                        width: Val::Px(550.0),
                        max_height: Val::Percent(85.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(24.0)),
                        row_gap: Val::Px(12.0),
                        overflow: Overflow::scroll_y(),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.06, 0.04, 0.12, 0.95)),
                ))
                .with_children(|panel| {
                    // Title
                    panel.spawn((
                        Text::new("Achievements"),
                        TextFont { font_size: 28.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.85, 0.3)),
                    ));

                    // Progress summary
                    panel.spawn((
                        Text::new(format!(
                            "{} / {} unlocked  |  Total bonus: +{:.0}%",
                            total_unlocked,
                            total_achievements,
                            (bonus - 1.0) * 100.0
                        )),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.75, 0.9)),
                    ));

                    // Divider
                    panel.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                        BackgroundColor(Color::srgba(1.0, 0.85, 0.3, 0.3)),
                    ));

                    // Achievement list
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(6.0),
                            ..default()
                        })
                        .with_children(|list| {
                            for id in AchievementId::ALL {
                                let owned = tracker.has(id);
                                let is_hidden = id.is_hidden() && !owned;

                                let bg_alpha = if owned { 0.15 } else { 0.05 };
                                let name_str = if is_hidden {
                                    "???".to_string()
                                } else {
                                    id.name().to_string()
                                };
                                let desc_str = if is_hidden {
                                    id.hidden_description().to_string()
                                } else {
                                    id.description().to_string()
                                };

                                let name_color = if owned {
                                    id.color()
                                } else if is_hidden {
                                    Color::srgba(0.5, 0.5, 0.5, 0.4)
                                } else {
                                    Color::srgba(0.7, 0.65, 0.75, 0.6)
                                };

                                let desc_color = if owned {
                                    Color::srgba(0.8, 0.75, 0.85, 0.7)
                                } else {
                                    Color::srgba(0.5, 0.48, 0.55, 0.5)
                                };

                                list.spawn(Node {
                                    width: Val::Percent(100.0),
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(8.0)),
                                    column_gap: Val::Px(12.0),
                                    border_radius: BorderRadius::all(Val::Px(4.0)),
                                    ..default()
                                })
                                .insert(BackgroundColor(Color::srgba(0.3, 0.25, 0.4, bg_alpha)))
                                .with_children(|row| {
                                    // Left: name + description
                                    row.spawn(Node {
                                        flex_direction: FlexDirection::Column,
                                        row_gap: Val::Px(2.0),
                                        flex_grow: 1.0,
                                        ..default()
                                    })
                                    .with_children(|info| {
                                        info.spawn((
                                            Text::new(name_str),
                                            TextFont { font_size: 16.0, ..default() },
                                            TextColor(name_color),
                                        ));
                                        info.spawn((
                                            Text::new(desc_str),
                                            TextFont { font_size: 12.0, ..default() },
                                            TextColor(desc_color),
                                        ));
                                    });

                                    // Right: reward badge
                                    if owned {
                                        row.spawn((
                                            Text::new(format!(
                                                "+{:.0}%",
                                                id.reward_multiplier() * 100.0
                                            )),
                                            TextFont { font_size: 14.0, ..default() },
                                            TextColor(Color::srgba(0.5, 1.0, 0.5, 0.8)),
                                        ));
                                    } else if !is_hidden {
                                        row.spawn((
                                            Text::new(format!(
                                                "+{:.0}%",
                                                id.reward_multiplier() * 100.0
                                            )),
                                            TextFont { font_size: 14.0, ..default() },
                                            TextColor(Color::srgba(0.5, 0.5, 0.5, 0.3)),
                                        ));
                                    }
                                });
                            }
                        });

                    // Footer
                    panel.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Px(1.0), margin: UiRect::top(Val::Px(8.0)), ..default() },
                        BackgroundColor(Color::srgba(1.0, 0.85, 0.3, 0.15)),
                    ));
                    panel.spawn((
                        Text::new("Press [V] to close"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgba(0.6, 0.55, 0.7, 0.5)),
                    ));
                });
        });
}

pub fn close_achievements(
    mut commands: Commands,
    panels: Query<Entity, With<AchievementsPanel>>,
) {
    for entity in &panels {
        commands.entity(entity).despawn();
    }
}
