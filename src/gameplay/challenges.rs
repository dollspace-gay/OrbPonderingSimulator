use super::generators::GeneratorState;
use super::state::GameState;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ========== CHALLENGE DEFINITIONS ==========

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChallengeId {
    /// No generators for 10 minutes → +5% all production permanently
    Silence,
    /// No clicking for 5 minutes → +10% passive generation permanently
    Blindfold,
    /// Survive with half wisdom scaling for 15 minutes → +8% click wisdom permanently
    Austerity,
    /// Generate 5 truths with no acolytes → +5% AFP earned permanently
    Solitude,
}

impl ChallengeId {
    pub const ALL: [ChallengeId; 4] = [
        Self::Silence,
        Self::Blindfold,
        Self::Austerity,
        Self::Solitude,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Silence => "Silence",
            Self::Blindfold => "Blindfold",
            Self::Austerity => "Austerity",
            Self::Solitude => "Solitude",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Silence => "Own no generators for 10 minutes.",
            Self::Blindfold => "Do not click the orb for 5 minutes.",
            Self::Austerity => "Endure double wisdom scaling for 15 minutes.",
            Self::Solitude => "Generate 5 truths with zero acolytes.",
        }
    }

    pub fn reward_description(&self) -> &'static str {
        match self {
            Self::Silence => "+5% all wisdom production",
            Self::Blindfold => "+10% passive generation",
            Self::Austerity => "+8% click wisdom",
            Self::Solitude => "+5% AFP earned per truth",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Silence => Color::srgb(0.4, 0.7, 0.9),
            Self::Blindfold => Color::srgb(0.6, 0.3, 0.8),
            Self::Austerity => Color::srgb(0.9, 0.6, 0.2),
            Self::Solitude => Color::srgb(0.3, 0.8, 0.5),
        }
    }

    /// Duration in seconds for timed challenges (None for goal-based)
    fn required_duration(&self) -> Option<f32> {
        match self {
            Self::Silence => Some(600.0),
            Self::Blindfold => Some(300.0),
            Self::Austerity => Some(900.0),
            Self::Solitude => None,
        }
    }
}

// ========== STATE ==========

#[derive(Resource, Debug)]
pub struct ChallengeState {
    /// Challenges completed (permanent)
    pub completed: Vec<ChallengeId>,
    /// Currently active challenge (if any)
    pub active: Option<ActiveChallenge>,
}

#[derive(Debug)]
pub struct ActiveChallenge {
    pub id: ChallengeId,
    /// Time elapsed since challenge started
    pub elapsed: f32,
    /// Whether the constraint has been violated
    pub failed: bool,
    /// Progress counter for goal-based challenges
    pub progress: u32,
}

impl Default for ChallengeState {
    fn default() -> Self {
        Self {
            completed: Vec::new(),
            active: None,
        }
    }
}

impl ChallengeState {
    pub fn has_completed(&self, id: ChallengeId) -> bool {
        self.completed.contains(&id)
    }

    pub fn is_active(&self) -> bool {
        self.active.is_some()
    }

    /// Permanent passive generation multiplier from completed challenges
    pub fn passive_multiplier(&self) -> f32 {
        let mut mult = 1.0;
        if self.has_completed(ChallengeId::Silence) {
            mult += 0.05;
        }
        if self.has_completed(ChallengeId::Blindfold) {
            mult += 0.10;
        }
        mult
    }

    /// Permanent click wisdom multiplier from completed challenges
    pub fn click_multiplier(&self) -> f32 {
        let mut mult = 1.0;
        if self.has_completed(ChallengeId::Austerity) {
            mult += 0.08;
        }
        mult
    }

    /// Permanent AFP bonus multiplier from completed challenges
    pub fn afp_multiplier(&self) -> f32 {
        if self.has_completed(ChallengeId::Solitude) {
            1.05
        } else {
            1.0
        }
    }

    /// Returns the wisdom scaling override if Austerity challenge is active
    pub fn active_scaling_override(&self) -> Option<f32> {
        match &self.active {
            Some(c) if c.id == ChallengeId::Austerity && !c.failed => Some(1.2),
            _ => None,
        }
    }
}

// ========== SYSTEMS ==========

/// Enforces challenge constraints and tracks progress
pub fn update_challenges(
    mut challenges: ResMut<ChallengeState>,
    mouse: Res<ButtonInput<MouseButton>>,
    generators: Res<GeneratorState>,
    time: Res<Time>,
    interactions: Query<&Interaction>,
) {
    let Some(ref mut active) = challenges.active else {
        return;
    };

    if active.failed {
        return;
    }

    active.elapsed += time.delta_secs();

    // Check constraint violations
    match active.id {
        ChallengeId::Silence => {
            let total_gens: u32 = generators.owned.iter().sum();
            if total_gens > 0 {
                active.failed = true;
            }
        }
        ChallengeId::Blindfold => {
            if mouse.just_pressed(MouseButton::Left) {
                // Only fail if not clicking UI buttons
                let clicking_ui = interactions
                    .iter()
                    .any(|i| *i == Interaction::Pressed);
                if !clicking_ui {
                    active.failed = true;
                }
            }
        }
        ChallengeId::Austerity => {
            // No constraint to enforce — the scaling override is applied elsewhere
        }
        ChallengeId::Solitude => {
            // Tracked via track_solitude_progress below
        }
    }

    // Check timed completion
    if let Some(duration) = active.id.required_duration() {
        if active.elapsed >= duration && !active.failed {
            let id = active.id;
            challenges.active = None;
            if !challenges.has_completed(id) {
                challenges.completed.push(id);
            }
        }
    }
}

/// Tracks truth generation for the Solitude challenge
pub fn track_solitude_progress(
    mut challenges: ResMut<ChallengeState>,
    wisdom: Res<super::wisdom::WisdomMeter>,
    acolytes: Res<super::acolytes::AcolyteState>,
    mut last_truths: Local<u32>,
) {
    let Some(ref mut active) = challenges.active else {
        *last_truths = wisdom.truths_generated;
        return;
    };

    if active.id != ChallengeId::Solitude || active.failed {
        *last_truths = wisdom.truths_generated;
        return;
    }

    // Fail if acolytes were summoned
    if acolytes.count > 0 {
        active.failed = true;
        *last_truths = wisdom.truths_generated;
        return;
    }

    // Track truths earned during challenge
    if wisdom.truths_generated > *last_truths {
        active.progress += wisdom.truths_generated - *last_truths;
    }
    *last_truths = wisdom.truths_generated;

    // Check completion
    if active.progress >= 5 {
        let id = active.id;
        challenges.active = None;
        if !challenges.has_completed(id) {
            challenges.completed.push(id);
        }
    }
}

/// Cancel active challenge on [C] key (also used to open challenge selection)
pub fn toggle_challenges(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut challenges: ResMut<ChallengeState>,
) {
    if keys.just_pressed(KeyCode::KeyC) {
        match current_state.get() {
            GameState::Playing => {
                if challenges.is_active() {
                    // Cancel active challenge
                    challenges.active = None;
                } else {
                    next_state.set(GameState::ChallengesOpen);
                }
            }
            GameState::ChallengesOpen => {
                next_state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}

// ========== UI: CHALLENGE SELECTION ==========

#[derive(Component)]
pub struct ChallengesPanel;

#[derive(Component)]
pub struct ChallengeButton(pub ChallengeId);

pub fn open_challenges(mut commands: Commands, challenges: Res<ChallengeState>) {
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
            ChallengesPanel,
        ))
        .with_children(|backdrop| {
            backdrop
                .spawn((
                    Node {
                        width: Val::Px(500.0),
                        max_height: Val::Percent(80.0),
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
                    panel.spawn((
                        Text::new("Meditation Challenges"),
                        TextFont { font_size: 26.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.75, 0.4)),
                    ));

                    panel.spawn((
                        Text::new("Test your discipline for permanent rewards."),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgba(0.7, 0.65, 0.8, 0.7)),
                    ));

                    // Divider
                    panel.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                        BackgroundColor(Color::srgba(0.9, 0.75, 0.4, 0.3)),
                    ));

                    // Challenge cards
                    for id in ChallengeId::ALL {
                        let completed = challenges.has_completed(id);
                        let challenge_color = id.color();

                        panel.spawn(Node {
                            width: Val::Percent(100.0),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(10.0)),
                            column_gap: Val::Px(12.0),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            ..default()
                        })
                        .insert(BackgroundColor(challenge_color.with_alpha(0.08)))
                        .with_children(|row| {
                            // Info
                            row.spawn(Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(2.0),
                                flex_grow: 1.0,
                                ..default()
                            })
                            .with_children(|info| {
                                let name_color = if completed {
                                    Color::srgba(0.5, 0.5, 0.5, 0.5)
                                } else {
                                    challenge_color
                                };
                                info.spawn((
                                    Text::new(id.name()),
                                    TextFont { font_size: 18.0, ..default() },
                                    TextColor(name_color),
                                ));
                                info.spawn((
                                    Text::new(id.description()),
                                    TextFont { font_size: 13.0, ..default() },
                                    TextColor(Color::srgba(0.7, 0.65, 0.75, 0.7)),
                                ));
                                info.spawn((
                                    Text::new(format!("Reward: {}", id.reward_description())),
                                    TextFont { font_size: 12.0, ..default() },
                                    TextColor(Color::srgba(0.5, 0.9, 0.5, 0.7)),
                                ));
                            });

                            // Button
                            if completed {
                                row.spawn((
                                    Text::new("Complete"),
                                    TextFont { font_size: 14.0, ..default() },
                                    TextColor(Color::srgba(0.4, 0.8, 0.4, 0.6)),
                                ));
                            } else {
                                row.spawn((
                                    Button,
                                    Node {
                                        padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        ..default()
                                    },
                                    BackgroundColor(challenge_color.with_alpha(0.7)),
                                    ChallengeButton(id),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("Begin"),
                                        TextFont { font_size: 14.0, ..default() },
                                        TextColor(Color::srgb(0.05, 0.03, 0.1)),
                                    ));
                                });
                            }
                        });
                    }

                    // Footer
                    panel.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Px(1.0), margin: UiRect::top(Val::Px(8.0)), ..default() },
                        BackgroundColor(Color::srgba(0.9, 0.75, 0.4, 0.15)),
                    ));
                    panel.spawn((
                        Text::new("Press [C] to close | [C] during challenge to cancel"),
                        TextFont { font_size: 13.0, ..default() },
                        TextColor(Color::srgba(0.6, 0.55, 0.7, 0.5)),
                    ));
                });
        });
}

pub fn close_challenges(mut commands: Commands, panels: Query<Entity, With<ChallengesPanel>>) {
    for entity in &panels {
        commands.entity(entity).despawn();
    }
}

/// Handles clicking a challenge begin button
pub fn handle_challenge_begin(
    interactions: Query<(&Interaction, &ChallengeButton), Changed<Interaction>>,
    mut challenges: ResMut<ChallengeState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if challenges.has_completed(button.0) || challenges.is_active() {
            continue;
        }

        challenges.active = Some(ActiveChallenge {
            id: button.0,
            elapsed: 0.0,
            failed: false,
            progress: 0,
        });
        next_state.set(GameState::Playing);
    }
}

// ========== HUD INDICATOR ==========

#[derive(Component)]
pub struct ChallengeIndicator;

/// Shows active challenge status in the HUD
pub fn render_challenge_indicator(
    mut commands: Commands,
    challenges: Res<ChallengeState>,
    existing: Query<Entity, With<ChallengeIndicator>>,
) {
    if !challenges.is_changed() {
        return;
    }

    for entity in &existing {
        commands.entity(entity).despawn();
    }

    let Some(ref active) = challenges.active else {
        return;
    };

    let status_text = if active.failed {
        "FAILED - Press [C] to cancel".to_string()
    } else if let Some(duration) = active.id.required_duration() {
        let remaining = (duration - active.elapsed).max(0.0);
        let mins = (remaining / 60.0) as u32;
        let secs = (remaining % 60.0) as u32;
        format!("{}:{:02} remaining", mins, secs)
    } else {
        format!("Progress: {}/5 truths", active.progress)
    };

    let status_color = if active.failed {
        Color::srgb(1.0, 0.3, 0.3)
    } else {
        active.id.color()
    };

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(90.0),
                left: Val::Px(16.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                padding: UiRect::all(Val::Px(8.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.03, 0.1, 0.8)),
            ChallengeIndicator,
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new(format!("Challenge: {}", active.id.name())),
                TextFont { font_size: 14.0, ..default() },
                TextColor(status_color),
            ));
            panel.spawn((
                Text::new(status_text),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgba(0.7, 0.65, 0.75, 0.7)),
            ));
        });
}

