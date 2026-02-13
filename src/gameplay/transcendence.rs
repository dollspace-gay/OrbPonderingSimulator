use super::state::GameState;
use super::wisdom::WisdomMeter;
use bevy::prelude::*;

// ========== DATA ==========

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnlightenmentId {
    /// +10% base wisdom per click permanently
    DeepRoots,
    /// +25% passive generation permanently
    EternalFlow,
    /// Start each run with 50 AFP
    HeadStart,
    /// +50% passive generation permanently
    CosmicResonance,
    /// Start each run with 200 AFP
    ArcaneInheritance,
    /// Moments of Clarity appear 2x as often
    ClarityAffinity,
    /// +100% all wisdom generation permanently
    Transcendent,
    /// Generators cost 10% less
    EfficientDesign,
}

impl EnlightenmentId {
    pub const ALL: [EnlightenmentId; 8] = [
        Self::DeepRoots,
        Self::EternalFlow,
        Self::HeadStart,
        Self::CosmicResonance,
        Self::ArcaneInheritance,
        Self::ClarityAffinity,
        Self::Transcendent,
        Self::EfficientDesign,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::DeepRoots => "Deep Roots",
            Self::EternalFlow => "Eternal Flow",
            Self::HeadStart => "Head Start",
            Self::CosmicResonance => "Cosmic Resonance",
            Self::ArcaneInheritance => "Arcane Inheritance",
            Self::ClarityAffinity => "Clarity Affinity",
            Self::Transcendent => "Transcendent Mind",
            Self::EfficientDesign => "Efficient Design",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::DeepRoots => "Your pondering echoes across lifetimes. (+10% click wisdom)",
            Self::EternalFlow => "Passive wisdom flows more freely. (+25% passive generation)",
            Self::HeadStart => "Begin each journey with arcane reserves. (Start with 50 AFP)",
            Self::CosmicResonance => {
                "The cosmos amplifies your meditation. (+50% passive generation)"
            }
            Self::ArcaneInheritance => "Greater reserves carry over. (Start with 200 AFP)",
            Self::ClarityAffinity => "Moments of Clarity find you more easily. (2x frequency)",
            Self::Transcendent => "Your mind operates on a higher plane. (+100% all wisdom)",
            Self::EfficientDesign => "Generators cost less to construct. (-10% generator costs)",
        }
    }

    pub fn cost(&self) -> u32 {
        match self {
            Self::DeepRoots => 1,
            Self::EternalFlow => 2,
            Self::HeadStart => 3,
            Self::CosmicResonance => 5,
            Self::ArcaneInheritance => 5,
            Self::ClarityAffinity => 8,
            Self::Transcendent => 15,
            Self::EfficientDesign => 4,
        }
    }
}

// ========== RESOURCES ==========

#[derive(Resource, Debug)]
pub struct TranscendenceState {
    pub insight: u32,
    pub total_transcendences: u32,
    pub purchased_enlightenments: Vec<EnlightenmentId>,
    /// Total wisdom accumulated in this run (for insight calculation)
    pub run_wisdom_accumulated: f64,
}

impl Default for TranscendenceState {
    fn default() -> Self {
        Self {
            insight: 0,
            total_transcendences: 0,
            purchased_enlightenments: Vec::new(),
            run_wisdom_accumulated: 0.0,
        }
    }
}

impl TranscendenceState {
    /// How much insight would be earned if transcending now
    pub fn pending_insight(&self) -> u32 {
        (self.run_wisdom_accumulated / 1000.0).sqrt().floor() as u32
    }

    pub fn has(&self, id: EnlightenmentId) -> bool {
        self.purchased_enlightenments.contains(&id)
    }

    /// Permanent click wisdom multiplier from enlightenments
    pub fn click_multiplier(&self) -> f32 {
        let mut mult = 1.0;
        if self.has(EnlightenmentId::DeepRoots) {
            mult += 0.1;
        }
        if self.has(EnlightenmentId::Transcendent) {
            mult += 1.0;
        }
        mult
    }

    /// Permanent passive wisdom multiplier from enlightenments
    pub fn passive_multiplier(&self) -> f32 {
        let mut mult = 1.0;
        if self.has(EnlightenmentId::EternalFlow) {
            mult += 0.25;
        }
        if self.has(EnlightenmentId::CosmicResonance) {
            mult += 0.5;
        }
        if self.has(EnlightenmentId::Transcendent) {
            mult += 1.0;
        }
        mult
    }

    /// Starting AFP from enlightenments
    pub fn starting_afp(&self) -> u64 {
        let mut afp = 0u64;
        if self.has(EnlightenmentId::HeadStart) {
            afp += 50;
        }
        if self.has(EnlightenmentId::ArcaneInheritance) {
            afp += 200;
        }
        afp
    }

    /// Generator cost discount (0.0 = no discount, 0.1 = 10% off)
    pub fn generator_cost_discount(&self) -> f64 {
        if self.has(EnlightenmentId::EfficientDesign) {
            0.1
        } else {
            0.0
        }
    }

    /// Moment spawn speed multiplier (higher = more frequent)
    pub fn clarity_frequency_multiplier(&self) -> f32 {
        if self.has(EnlightenmentId::ClarityAffinity) {
            2.0
        } else {
            1.0
        }
    }
}

/// System to accumulate run wisdom from all sources
pub fn accumulate_run_wisdom(
    wisdom: Res<WisdomMeter>,
    mut transcendence: ResMut<TranscendenceState>,
    mut last_wisdom: Local<f32>,
) {
    let current = wisdom.current;
    if current < *last_wisdom {
        // Wisdom was reset (truth generated) - the amount that was consumed is the max wisdom
        transcendence.run_wisdom_accumulated += wisdom.max_wisdom as f64;
    }
    *last_wisdom = current;
}

// ========== TRANSCENDENCE UI ==========

#[derive(Component)]
pub struct TranscendencePanel;

#[derive(Component)]
pub struct TranscendButton;

#[derive(Component)]
pub struct EnlightenmentBuyButton(pub EnlightenmentId);

#[derive(Component)]
pub struct EnlightenmentPanel;

#[derive(Component)]
pub struct InsightText;

pub fn toggle_transcendence(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::TranscendenceOpen),
            GameState::TranscendenceOpen => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

pub fn open_transcendence_ui(mut commands: Commands, transcendence: Res<TranscendenceState>) {
    let pending = transcendence.pending_insight();

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
            TranscendencePanel,
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
                        Text::new("Transcendence"),
                        TextFont { font_size: 28.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.5, 1.0)),
                    ));

                    // Divider
                    panel.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                        BackgroundColor(Color::srgba(0.7, 0.5, 1.0, 0.3)),
                    ));

                    // Insight display
                    panel.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.0),
                        ..default()
                    }).with_children(|section| {
                        section.spawn((
                            Text::new(format!(
                                "Current Insight: {}  |  Transcendences: {}",
                                transcendence.insight, transcendence.total_transcendences
                            )),
                            TextFont { font_size: 18.0, ..default() },
                            TextColor(Color::srgb(0.9, 0.8, 1.0)),
                            InsightText,
                        ));

                        let insight_msg = if pending > 0 {
                            format!("Transcending now would grant +{} Insight", pending)
                        } else {
                            "Accumulate more wisdom to earn Insight...".to_string()
                        };
                        section.spawn((
                            Text::new(insight_msg),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(Color::srgba(0.7, 0.6, 0.9, 0.8)),
                        ));

                        section.spawn((
                            Text::new("Transcendence resets AFP, generators, shop upgrades, and acolytes.\nInsight and enlightenments are permanent."),
                            TextFont { font_size: 13.0, ..default() },
                            TextColor(Color::srgba(0.6, 0.55, 0.7, 0.6)),
                        ));
                    });

                    // Transcend button (only if pending > 0)
                    if pending > 0 {
                        panel.spawn((
                            Button,
                            Node {
                                padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.7, 0.4, 1.0, 0.9)),
                            TranscendButton,
                        )).with_children(|btn| {
                            btn.spawn((
                                Text::new(format!("Transcend (+{} Insight)", pending)),
                                TextFont { font_size: 18.0, ..default() },
                                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                            ));
                        });
                    }

                    // Divider
                    panel.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                        BackgroundColor(Color::srgba(0.7, 0.5, 1.0, 0.15)),
                    ));

                    // Enlightenment upgrades header
                    panel.spawn((
                        Text::new("Enlightenment Upgrades"),
                        TextFont { font_size: 20.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.8, 1.0)),
                    ));

                    // Enlightenment items
                    panel.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        },
                        EnlightenmentPanel,
                    )).with_children(|list| {
                        for eid in EnlightenmentId::ALL {
                            let owned = transcendence.has(eid);
                            let affordable = transcendence.insight >= eid.cost();

                            list.spawn(Node {
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(8.0)),
                                column_gap: Val::Px(12.0),
                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                ..default()
                            }).with_children(|row| {
                                row.spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(2.0),
                                    flex_grow: 1.0,
                                    ..default()
                                }).with_children(|info| {
                                    let name_color = if owned {
                                        Color::srgba(0.6, 0.55, 0.7, 0.5)
                                    } else {
                                        Color::srgb(0.9, 0.88, 0.8)
                                    };
                                    info.spawn((
                                        Text::new(eid.name()),
                                        TextFont { font_size: 18.0, ..default() },
                                        TextColor(name_color),
                                    ));
                                    info.spawn((
                                        Text::new(eid.description()),
                                        TextFont { font_size: 13.0, ..default() },
                                        TextColor(Color::srgba(0.6, 0.55, 0.7, 0.7)),
                                    ));
                                });

                                let (btn_bg, btn_text_color, label) = if owned {
                                    (
                                        Color::srgba(0.2, 0.5, 0.25, 0.6),
                                        Color::srgb(0.4, 0.9, 0.5),
                                        "Owned".to_string(),
                                    )
                                } else if affordable {
                                    (
                                        Color::srgba(0.7, 0.5, 1.0, 0.9),
                                        Color::srgb(1.0, 1.0, 1.0),
                                        format!("{} Insight", eid.cost()),
                                    )
                                } else {
                                    (
                                        Color::srgba(0.3, 0.25, 0.4, 0.5),
                                        Color::srgba(0.5, 0.45, 0.6, 0.5),
                                        format!("{} Insight", eid.cost()),
                                    )
                                };

                                row.spawn((
                                    Button,
                                    Node {
                                        padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        justify_content: JustifyContent::Center,
                                        min_width: Val::Px(90.0),
                                        ..default()
                                    },
                                    BackgroundColor(btn_bg),
                                    EnlightenmentBuyButton(eid),
                                )).with_children(|btn| {
                                    btn.spawn((
                                        Text::new(label),
                                        TextFont { font_size: 14.0, ..default() },
                                        TextColor(btn_text_color),
                                    ));
                                });
                            });
                        }
                    });

                    // Footer
                    panel.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Px(1.0), margin: UiRect::top(Val::Px(8.0)), ..default() },
                        BackgroundColor(Color::srgba(0.7, 0.5, 1.0, 0.15)),
                    ));
                    panel.spawn((
                        Text::new("Press [T] to close"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgba(0.6, 0.55, 0.7, 0.5)),
                    ));
                });
        });
}

pub fn close_transcendence_ui(
    mut commands: Commands,
    panels: Query<Entity, With<TranscendencePanel>>,
) {
    for entity in &panels {
        commands.entity(entity).despawn();
    }
}

pub fn handle_transcend_click(
    interactions: Query<&Interaction, (Changed<Interaction>, With<TranscendButton>)>,
    mut transcendence: ResMut<TranscendenceState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let gained = transcendence.pending_insight();
        if gained == 0 {
            continue;
        }

        // Grant insight (permanent)
        transcendence.insight += gained;
        transcendence.total_transcendences += 1;
        transcendence.run_wisdom_accumulated = 0.0;

        // Go to school selection — the actual reset happens when a school is chosen
        next_state.set(GameState::SchoolSelection);
    }
}

pub fn handle_enlightenment_buy(
    interactions: Query<(&Interaction, &EnlightenmentBuyButton), Changed<Interaction>>,
    mut transcendence: ResMut<TranscendenceState>,
) {
    for (interaction, button) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if transcendence.has(button.0) {
            continue;
        }

        let cost = button.0.cost();
        if transcendence.insight < cost {
            continue;
        }

        transcendence.insight -= cost;
        transcendence.purchased_enlightenments.push(button.0);
    }
}
