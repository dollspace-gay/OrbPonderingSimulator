use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SchoolOfThought {
    #[default]
    None,
    Stoicism,
    Mysticism,
    Empiricism,
    Nihilism,
}

impl SchoolOfThought {
    pub const CHOOSABLE: [SchoolOfThought; 4] = [
        Self::Stoicism,
        Self::Mysticism,
        Self::Empiricism,
        Self::Nihilism,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "Unaligned",
            Self::Stoicism => "Stoicism",
            Self::Mysticism => "Mysticism",
            Self::Empiricism => "Empiricism",
            Self::Nihilism => "Nihilism",
        }
    }

    pub fn subtitle(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Stoicism => "The Way of Stillness",
            Self::Mysticism => "The Way of Visions",
            Self::Empiricism => "The Way of Method",
            Self::Nihilism => "The Way of the Void",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::None => "No school chosen.",
            Self::Stoicism => "Passive generation +50%. Wisdom scaling reduced to 1.07x. The patient mind sees furthest.",
            Self::Mysticism => "Moments of Clarity appear 2x as often. Buff durations +50%. Wisdom bursts doubled. Embrace the unknowable.",
            Self::Empiricism => "Click wisdom +75%. AFP per truth +30%. Each observation builds the next. Knowledge compounds.",
            Self::Nihilism => "All generation starts at 0.5x. Each truth generated adds +5% generation. From nothing, everything accelerates.",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::None => Color::srgba(0.5, 0.5, 0.5, 0.5),
            Self::Stoicism => Color::srgb(0.4, 0.7, 0.9),
            Self::Mysticism => Color::srgb(0.8, 0.4, 1.0),
            Self::Empiricism => Color::srgb(1.0, 0.75, 0.3),
            Self::Nihilism => Color::srgb(0.6, 0.2, 0.2),
        }
    }
}

/// Tracks the active school and run-specific school state
#[derive(Resource, Debug, Default)]
pub struct SchoolState {
    pub active: SchoolOfThought,
    /// Truths generated this run (for Nihilism scaling)
    pub run_truths: u32,
}

impl SchoolState {
    /// Click wisdom multiplier from the active school
    pub fn click_multiplier(&self) -> f32 {
        match self.active {
            SchoolOfThought::Empiricism => 1.75,
            _ => 1.0,
        }
    }

    /// Passive generation multiplier from the active school
    pub fn passive_multiplier(&self) -> f32 {
        match self.active {
            SchoolOfThought::Stoicism => 1.5,
            SchoolOfThought::Nihilism => {
                // Starts at 0.5x, +5% per truth
                0.5 + 0.05 * self.run_truths as f32
            }
            _ => 1.0,
        }
    }

    /// Wisdom scaling factor override (None = use default)
    pub fn scaling_override(&self) -> Option<f32> {
        match self.active {
            SchoolOfThought::Stoicism => Some(1.07),
            _ => None,
        }
    }

    /// Extra AFP bonus per truth (added to base + tracker bonus)
    pub fn afp_bonus_per_truth(&self) -> u64 {
        match self.active {
            // +30% of base (10 AFP) = +3 AFP per truth
            SchoolOfThought::Empiricism => 3,
            _ => 0,
        }
    }

    /// Moment spawn frequency multiplier (higher = more frequent)
    pub fn moment_frequency_multiplier(&self) -> f32 {
        match self.active {
            SchoolOfThought::Mysticism => 2.0,
            _ => 1.0,
        }
    }

    /// Moment buff duration multiplier
    pub fn moment_duration_multiplier(&self) -> f32 {
        match self.active {
            SchoolOfThought::Mysticism => 1.5,
            _ => 1.0,
        }
    }

    /// Moment wisdom burst multiplier
    pub fn moment_burst_multiplier(&self) -> f64 {
        match self.active {
            SchoolOfThought::Mysticism => 2.0,
            _ => 1.0,
        }
    }
}

// ========== SCHOOL SELECTION UI ==========

#[derive(Component)]
pub struct SchoolSelectionPanel;

#[derive(Component)]
pub struct SchoolChoiceButton(pub SchoolOfThought);

pub fn open_school_selection(mut commands: Commands) {
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
            SchoolSelectionPanel,
        ))
        .with_children(|backdrop| {
            backdrop
                .spawn(Node {
                    width: Val::Px(650.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(24.0)),
                    row_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                })
                .insert(BackgroundColor(Color::srgba(0.06, 0.04, 0.12, 0.95)))
                .with_children(|panel| {
                    panel.spawn((
                        Text::new("Choose Your School of Thought"),
                        TextFont { font_size: 26.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.8, 1.0)),
                    ));

                    panel.spawn((
                        Text::new("Your school shapes this run's strengths. Choose wisely."),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgba(0.7, 0.65, 0.8, 0.7)),
                    ));

                    // Divider
                    panel.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
                        BackgroundColor(Color::srgba(0.7, 0.5, 1.0, 0.3)),
                    ));

                    // School cards
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|list| {
                            for school in SchoolOfThought::CHOOSABLE {
                                let school_color = school.color();

                                list.spawn((
                                    Button,
                                    Node {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Column,
                                        padding: UiRect::all(Val::Px(14.0)),
                                        row_gap: Val::Px(4.0),
                                        border_radius: BorderRadius::all(Val::Px(6.0)),
                                        ..default()
                                    },
                                    BackgroundColor(school_color.with_alpha(0.15)),
                                    SchoolChoiceButton(school),
                                ))
                                .with_children(|card| {
                                    // School name + subtitle
                                    card.spawn(Node {
                                        column_gap: Val::Px(12.0),
                                        align_items: AlignItems::Baseline,
                                        ..default()
                                    })
                                    .with_children(|header| {
                                        header.spawn((
                                            Text::new(school.name()),
                                            TextFont { font_size: 20.0, ..default() },
                                            TextColor(school_color),
                                        ));
                                        header.spawn((
                                            Text::new(school.subtitle()),
                                            TextFont { font_size: 14.0, ..default() },
                                            TextColor(school_color.with_alpha(0.6)),
                                        ));
                                    });

                                    // Description
                                    card.spawn((
                                        Text::new(school.description()),
                                        TextFont { font_size: 13.0, ..default() },
                                        TextColor(Color::srgba(0.8, 0.75, 0.85, 0.8)),
                                    ));
                                });
                            }
                        });
                });
        });
}

pub fn close_school_selection(
    mut commands: Commands,
    panels: Query<Entity, With<SchoolSelectionPanel>>,
) {
    for entity in &panels {
        commands.entity(entity).despawn();
    }
}
