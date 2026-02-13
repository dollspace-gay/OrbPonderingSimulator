use super::state::GameState;
use super::wisdom::TruthGenerated;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// ========== TRUTH CATEGORIES ==========

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TruthCategory {
    OriginalTruths,
    CosmicMusings,
    OrbPhilosophy,
    ArcaneObservations,
    ExistentialWisdom,
    AcolyteWisdom,
    NatureAndElements,
    TimeAndPatience,
    FoodForThought,
    DeepNonsense,
    PhilosophicalMusings,
    TowerAndSanctum,
    CatsAndFamiliars,
    CandlesAndLight,
    SoundsAndSilence,
    BooksAndKnowledge,
    DreamsAndSleep,
}

impl TruthCategory {
    pub const ALL: [TruthCategory; 17] = [
        Self::OriginalTruths,
        Self::CosmicMusings,
        Self::OrbPhilosophy,
        Self::ArcaneObservations,
        Self::ExistentialWisdom,
        Self::AcolyteWisdom,
        Self::NatureAndElements,
        Self::TimeAndPatience,
        Self::FoodForThought,
        Self::DeepNonsense,
        Self::PhilosophicalMusings,
        Self::TowerAndSanctum,
        Self::CatsAndFamiliars,
        Self::CandlesAndLight,
        Self::SoundsAndSilence,
        Self::BooksAndKnowledge,
        Self::DreamsAndSleep,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::OriginalTruths => "Original Truths",
            Self::CosmicMusings => "Cosmic Musings",
            Self::OrbPhilosophy => "Orb Philosophy",
            Self::ArcaneObservations => "Arcane Observations",
            Self::ExistentialWisdom => "Existential Wisdom",
            Self::AcolyteWisdom => "Acolyte Wisdom",
            Self::NatureAndElements => "Nature & Elements",
            Self::TimeAndPatience => "Time & Patience",
            Self::FoodForThought => "Food for Thought",
            Self::DeepNonsense => "Deep Nonsense",
            Self::PhilosophicalMusings => "Philosophical Musings",
            Self::TowerAndSanctum => "Tower & Sanctum",
            Self::CatsAndFamiliars => "Cats & Familiars",
            Self::CandlesAndLight => "Candles & Light",
            Self::SoundsAndSilence => "Sounds & Silence",
            Self::BooksAndKnowledge => "Books & Knowledge",
            Self::DreamsAndSleep => "Dreams & Sleep",
        }
    }

    /// Inclusive index range (start, end) into DEEP_TRUTHS
    pub fn index_range(&self) -> (usize, usize) {
        match self {
            Self::OriginalTruths => (0, 17),
            Self::CosmicMusings => (18, 41),
            Self::OrbPhilosophy => (42, 67),
            Self::ArcaneObservations => (68, 92),
            Self::ExistentialWisdom => (93, 115),
            Self::AcolyteWisdom => (116, 131),
            Self::NatureAndElements => (132, 155),
            Self::TimeAndPatience => (156, 172),
            Self::FoodForThought => (173, 192),
            Self::DeepNonsense => (193, 216),
            Self::PhilosophicalMusings => (217, 231),
            Self::TowerAndSanctum => (232, 241),
            Self::CatsAndFamiliars => (242, 251),
            Self::CandlesAndLight => (252, 261),
            Self::SoundsAndSilence => (262, 271),
            Self::BooksAndKnowledge => (272, 281),
            Self::DreamsAndSleep => (282, 291),
        }
    }

    pub fn count(&self) -> usize {
        let (start, end) = self.index_range();
        end - start + 1
    }

    pub fn color(&self) -> Color {
        match self {
            Self::OriginalTruths => Color::srgb(0.8, 0.7, 1.0),
            Self::CosmicMusings => Color::srgb(0.4, 0.6, 1.0),
            Self::OrbPhilosophy => Color::srgb(0.7, 0.4, 1.0),
            Self::ArcaneObservations => Color::srgb(1.0, 0.5, 0.3),
            Self::ExistentialWisdom => Color::srgb(0.6, 0.8, 0.6),
            Self::AcolyteWisdom => Color::srgb(0.5, 0.9, 0.7),
            Self::NatureAndElements => Color::srgb(0.3, 0.8, 0.4),
            Self::TimeAndPatience => Color::srgb(0.9, 0.8, 0.5),
            Self::FoodForThought => Color::srgb(1.0, 0.6, 0.4),
            Self::DeepNonsense => Color::srgb(1.0, 0.4, 0.7),
            Self::PhilosophicalMusings => Color::srgb(0.7, 0.7, 0.9),
            Self::TowerAndSanctum => Color::srgb(0.6, 0.5, 0.4),
            Self::CatsAndFamiliars => Color::srgb(0.9, 0.7, 0.5),
            Self::CandlesAndLight => Color::srgb(1.0, 0.9, 0.4),
            Self::SoundsAndSilence => Color::srgb(0.5, 0.7, 0.9),
            Self::BooksAndKnowledge => Color::srgb(0.7, 0.6, 0.5),
            Self::DreamsAndSleep => Color::srgb(0.6, 0.5, 0.9),
        }
    }

    pub fn category_for_index(index: usize) -> Option<TruthCategory> {
        for cat in Self::ALL {
            let (start, end) = cat.index_range();
            if index >= start && index <= end {
                return Some(cat);
            }
        }
        None
    }
}

// ========== TRUTH CODEX RESOURCE ==========

#[derive(Resource, Debug, Default)]
pub struct TruthCodex {
    pub discovered: HashSet<usize>,
    pub completed_categories: Vec<TruthCategory>,
    pub notification_queue: Vec<TruthCategory>,
}

impl TruthCodex {
    /// Record a truth discovery. Returns true if this was a new discovery.
    pub fn discover(&mut self, index: usize) -> bool {
        if !self.discovered.insert(index) {
            return false;
        }

        // Check if this completes a category
        if let Some(cat) = TruthCategory::category_for_index(index) {
            if !self.completed_categories.contains(&cat) && self.is_category_complete(cat) {
                self.completed_categories.push(cat);
                self.notification_queue.push(cat);
            }
        }

        true
    }

    /// How many truths discovered in a category
    pub fn category_progress(&self, cat: TruthCategory) -> usize {
        let (start, end) = cat.index_range();
        self.discovered.iter().filter(|&&i| i >= start && i <= end).count()
    }

    pub fn is_category_complete(&self, cat: TruthCategory) -> bool {
        self.category_progress(cat) >= cat.count()
    }

    /// Permanent wisdom multiplier: 1.0 + 0.05 per completed category
    pub fn wisdom_multiplier(&self) -> f32 {
        1.0 + self.completed_categories.len() as f32 * 0.05
    }
}

// ========== SYSTEMS ==========

/// Track truth discoveries from TruthGenerated messages
pub fn track_truth_discovery(
    mut codex: ResMut<TruthCodex>,
    mut truth_messages: MessageReader<TruthGenerated>,
) {
    for msg in truth_messages.read() {
        // Skip sentinel values (e.g. dream truths use usize::MAX)
        if msg.truth_index < 292 {
            codex.discover(msg.truth_index);
        }
    }
}

/// Toggle codex panel with [X] key
pub fn toggle_codex(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyX) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::CodexOpen),
            GameState::CodexOpen => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

// ========== CODEX UI ==========

#[derive(Component)]
pub struct CodexPanel;

pub fn open_codex(mut commands: Commands, codex: Res<TruthCodex>) {
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            CodexPanel,
        ))
        .with_children(|backdrop| {
            backdrop
                .spawn((
                    Node {
                        width: Val::Px(600.0),
                        max_height: Val::Percent(85.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(24.0)),
                        row_gap: Val::Px(12.0),
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.06, 0.04, 0.12, 0.95)),
                ))
                .with_children(|panel| {
                    // Title
                    let total_discovered = codex.discovered.len();
                    let completed = codex.completed_categories.len();
                    let bonus_pct = (completed as f32 * 5.0) as u32;

                    panel.spawn((
                        Text::new("Truth Codex"),
                        TextFont { font_size: 26.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.8, 1.0)),
                    ));

                    panel.spawn((
                        Text::new(format!(
                            "{}/292 truths discovered | {}/17 categories complete | +{}% wisdom",
                            total_discovered, completed, bonus_pct
                        )),
                        TextFont { font_size: 13.0, ..default() },
                        TextColor(Color::srgba(0.7, 0.65, 0.8, 0.7)),
                    ));

                    // Divider
                    panel.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                        BackgroundColor(Color::srgba(0.7, 0.5, 1.0, 0.3)),
                    ));

                    // Category rows
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|list| {
                            for cat in TruthCategory::ALL {
                                let progress = codex.category_progress(cat);
                                let total = cat.count();
                                let complete = codex.is_category_complete(cat);
                                let cat_color = cat.color();

                                list.spawn(Node {
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::all(Val::Px(10.0)),
                                    row_gap: Val::Px(4.0),
                                    border_radius: BorderRadius::all(Val::Px(4.0)),
                                    ..default()
                                })
                                .insert(BackgroundColor(if complete {
                                    cat_color.with_alpha(0.2)
                                } else {
                                    Color::srgba(0.1, 0.08, 0.15, 0.5)
                                }))
                                .with_children(|row| {
                                    // Header: name + count
                                    row.spawn(Node {
                                        width: Val::Percent(100.0),
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    })
                                    .with_children(|header| {
                                        let label = if complete {
                                            format!("{} [COMPLETE]", cat.name())
                                        } else {
                                            cat.name().to_string()
                                        };
                                        header.spawn((
                                            Text::new(label),
                                            TextFont { font_size: 16.0, ..default() },
                                            TextColor(if complete {
                                                cat_color
                                            } else {
                                                cat_color.with_alpha(0.7)
                                            }),
                                        ));
                                        header.spawn((
                                            Text::new(format!("{}/{}", progress, total)),
                                            TextFont { font_size: 14.0, ..default() },
                                            TextColor(Color::srgba(0.7, 0.7, 0.8, 0.6)),
                                        ));
                                    });

                                    // Progress bar
                                    let pct = if total > 0 {
                                        progress as f32 / total as f32 * 100.0
                                    } else {
                                        0.0
                                    };
                                    row.spawn((
                                        Node {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(6.0),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgba(0.2, 0.15, 0.3, 0.5)),
                                    ))
                                    .with_children(|bar_bg| {
                                        bar_bg.spawn((
                                            Node {
                                                width: Val::Percent(pct),
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            BackgroundColor(if complete {
                                                cat_color
                                            } else {
                                                cat_color.with_alpha(0.6)
                                            }),
                                        ));
                                    });

                                    // Bonus text
                                    if complete {
                                        row.spawn((
                                            Text::new("+5% permanent wisdom bonus"),
                                            TextFont { font_size: 11.0, ..default() },
                                            TextColor(Color::srgb(0.5, 1.0, 0.6)),
                                        ));
                                    }
                                });
                            }
                        });

                    // Close hint
                    panel.spawn((
                        Text::new("[X] Close"),
                        TextFont { font_size: 13.0, ..default() },
                        TextColor(Color::srgba(0.6, 0.6, 0.7, 0.5)),
                    ));
                });
        });
}

pub fn close_codex(mut commands: Commands, panels: Query<Entity, With<CodexPanel>>) {
    for entity in &panels {
        commands.entity(entity).despawn();
    }
}

// ========== NOTIFICATIONS ==========

#[derive(Component)]
pub struct CodexNotification {
    pub timer: f32,
}

pub fn spawn_codex_notifications(
    mut commands: Commands,
    mut codex: ResMut<TruthCodex>,
) {
    while let Some(cat) = codex.notification_queue.pop() {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(120.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                CodexNotification { timer: 4.0 },
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        },
                        BackgroundColor(cat.color().with_alpha(0.9)),
                    ))
                    .with_children(|badge| {
                        badge.spawn((
                            Text::new(format!(
                                "Codex Complete: {} (+5% wisdom)",
                                cat.name()
                            )),
                            TextFont { font_size: 18.0, ..default() },
                            TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        ));
                    });
            });
    }
}

pub fn update_codex_notifications(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CodexNotification)>,
    time: Res<Time>,
) {
    for (entity, mut notif) in &mut query {
        notif.timer -= time.delta_secs();
        if notif.timer <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
