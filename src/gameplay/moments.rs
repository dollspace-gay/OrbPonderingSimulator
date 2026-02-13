use super::generators::GeneratorState;
use super::progression::ArcaneProgress;
use super::shop::PurchaseTracker;
use super::transcendence::TranscendenceState;
use super::wisdom::WisdomMeter;
use bevy::prelude::*;
use rand::Rng;

/// The different bonus effects a Moment of Clarity can grant
#[derive(Debug, Clone, Copy)]
pub enum MomentEffect {
    /// Instantly adds a burst of wisdom
    WisdomBurst,
    /// Doubles all wisdom generation for a duration
    WisdomMultiplier,
    /// Grants bonus AFP instantly
    AfpBonus,
    /// Triples click power for a duration
    ClickFrenzy,
}

impl MomentEffect {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Self::WisdomBurst,
            1 => Self::WisdomMultiplier,
            2 => Self::AfpBonus,
            _ => Self::ClickFrenzy,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::WisdomBurst => "Sudden Insight",
            Self::WisdomMultiplier => "Heightened Awareness",
            Self::AfpBonus => "Arcane Windfall",
            Self::ClickFrenzy => "Pondering Frenzy",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::WisdomBurst => "A flash of pure wisdom floods your mind.",
            Self::WisdomMultiplier => "All wisdom flows twice as fast for 30 seconds.",
            Self::AfpBonus => "The arcane currents deliver a gift of focus points.",
            Self::ClickFrenzy => "Your pondering intensifies threefold for 20 seconds.",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::WisdomBurst => Color::srgb(0.6, 0.4, 1.0),
            Self::WisdomMultiplier => Color::srgb(0.3, 0.9, 1.0),
            Self::AfpBonus => Color::srgb(1.0, 0.85, 0.3),
            Self::ClickFrenzy => Color::srgb(1.0, 0.5, 0.3),
        }
    }
}

/// Tracks the state of the Moments of Clarity system
#[derive(Resource)]
pub struct MomentState {
    /// Timer until the next moment spawns
    pub spawn_timer: Timer,
    /// Currently active moment (if any, waiting to be clicked)
    pub pending: Option<PendingMoment>,
    /// Currently active buff from a claimed moment
    pub active_buff: Option<ActiveBuff>,
}

pub struct PendingMoment {
    pub effect: MomentEffect,
    pub lifetime: Timer,
}

pub struct ActiveBuff {
    pub effect: MomentEffect,
    pub timer: Timer,
}

impl Default for MomentState {
    fn default() -> Self {
        let initial_delay = rand::thread_rng().gen_range(180.0..420.0);
        Self {
            spawn_timer: Timer::from_seconds(initial_delay, TimerMode::Once),
            pending: None,
            active_buff: None,
        }
    }
}

impl MomentState {
    fn reset_spawn_timer(&mut self, frequency_multiplier: f32) {
        let delay = rand::thread_rng().gen_range(300.0..900.0) / frequency_multiplier;
        self.spawn_timer = Timer::from_seconds(delay, TimerMode::Once);
    }

    /// Returns the current wisdom multiplier from active buffs (1.0 = no buff)
    pub fn wisdom_multiplier(&self) -> f32 {
        match &self.active_buff {
            Some(buff) if matches!(buff.effect, MomentEffect::WisdomMultiplier) => 2.0,
            _ => 1.0,
        }
    }

    /// Returns the current click multiplier from active buffs (1.0 = no buff)
    pub fn click_multiplier(&self) -> f32 {
        match &self.active_buff {
            Some(buff) if matches!(buff.effect, MomentEffect::ClickFrenzy) => 3.0,
            _ => 1.0,
        }
    }
}

/// Ticks timers and spawns new moments
pub fn update_moments(
    mut moments: ResMut<MomentState>,
    time: Res<Time>,
    transcendence: Res<TranscendenceState>,
) {
    // Tick spawn timer
    if moments.pending.is_none() {
        moments.spawn_timer.tick(time.delta());
        if moments.spawn_timer.just_finished() {
            let effect = MomentEffect::random();
            moments.pending = Some(PendingMoment {
                effect,
                lifetime: Timer::from_seconds(30.0, TimerMode::Once),
            });
        }
    }

    // Tick pending moment lifetime (auto-dismiss if not clicked)
    let mut expired = false;
    if let Some(pending) = &mut moments.pending {
        pending.lifetime.tick(time.delta());
        if pending.lifetime.just_finished() {
            expired = true;
        }
    }
    if expired {
        moments.pending = None;
        moments.reset_spawn_timer(transcendence.clarity_frequency_multiplier());
    }

    // Tick active buff timer
    let mut buff_expired = false;
    if let Some(buff) = &mut moments.active_buff {
        buff.timer.tick(time.delta());
        if buff.timer.just_finished() {
            buff_expired = true;
        }
    }
    if buff_expired {
        moments.active_buff = None;
    }
}

// ========== UI ==========

#[derive(Component)]
pub struct MomentPopup;

#[derive(Component)]
pub struct MomentClickArea;

#[derive(Component)]
pub struct BuffIndicator;

/// Spawns/despawns the clickable moment popup
pub fn render_moment_popup(
    mut commands: Commands,
    moments: Res<MomentState>,
    existing: Query<Entity, With<MomentPopup>>,
) {
    if !moments.is_changed() {
        return;
    }

    // Despawn existing popup
    for entity in &existing {
        commands.entity(entity).despawn();
    }

    // Spawn new popup if there's a pending moment
    let Some(pending) = &moments.pending else {
        return;
    };

    let effect_color = pending.effect.color();

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(25.0),
                left: Val::Percent(50.0),
                margin: UiRect::left(Val::Px(-150.0)),
                width: Val::Px(300.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(8.0),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.03, 0.12, 0.9)),
            MomentPopup,
        ))
        .with_children(|popup| {
            // Glow label
            popup.spawn((
                Text::new("~ Moment of Clarity ~"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
            ));

            // Effect name
            popup.spawn((
                Text::new(pending.effect.label()),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(effect_color),
            ));

            // Description
            popup.spawn((
                Text::new(pending.effect.description()),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgba(0.8, 0.78, 0.7, 0.8)),
            ));

            // Click button
            popup
                .spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        margin: UiRect::top(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(effect_color.with_alpha(0.8)),
                    MomentClickArea,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Embrace"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.05, 0.03, 0.1)),
                    ));
                });

            // Remaining time hint
            let remaining = pending.lifetime.remaining_secs();
            popup.spawn((
                Text::new(format!("Fades in {:.0}s", remaining)),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgba(0.5, 0.5, 0.6, 0.5)),
            ));
        });
}

/// Handles clicking the Moment of Clarity button
pub fn handle_moment_click(
    interactions: Query<&Interaction, (Changed<Interaction>, With<MomentClickArea>)>,
    mut moments: ResMut<MomentState>,
    mut wisdom: ResMut<WisdomMeter>,
    mut progress: ResMut<ArcaneProgress>,
    generators: Res<GeneratorState>,
    tracker: Res<PurchaseTracker>,
    transcendence: Res<TranscendenceState>,
) {
    for interaction in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Some(pending) = moments.pending.take() else {
            continue;
        };

        match pending.effect {
            MomentEffect::WisdomBurst => {
                // Grant 10x current per-second production as instant wisdom, minimum 5.0
                let base_prod = generators.total_base_production();
                let rate = base_prod
                    * (1.0 + tracker.efficiency_bonus as f64)
                    * tracker.wisdom_speed_bonus as f64;
                let burst = (rate * 10.0).max(5.0);
                wisdom.current += burst as f32;
            }
            MomentEffect::WisdomMultiplier => {
                moments.active_buff = Some(ActiveBuff {
                    effect: MomentEffect::WisdomMultiplier,
                    timer: Timer::from_seconds(30.0, TimerMode::Once),
                });
            }
            MomentEffect::AfpBonus => {
                // Grant 20% of current AFP or minimum 15
                let bonus = (progress.focus_points / 5).max(15);
                progress.focus_points += bonus;
            }
            MomentEffect::ClickFrenzy => {
                moments.active_buff = Some(ActiveBuff {
                    effect: MomentEffect::ClickFrenzy,
                    timer: Timer::from_seconds(20.0, TimerMode::Once),
                });
            }
        }

        moments.reset_spawn_timer(transcendence.clarity_frequency_multiplier());
    }
}

/// Shows a buff indicator in the HUD when a buff is active
pub fn render_buff_indicator(
    mut commands: Commands,
    moments: Res<MomentState>,
    existing: Query<Entity, With<BuffIndicator>>,
) {
    if !moments.is_changed() {
        return;
    }

    for entity in &existing {
        commands.entity(entity).despawn();
    }

    let Some(buff) = &moments.active_buff else {
        return;
    };

    let remaining = buff.timer.remaining_secs();
    let label = match buff.effect {
        MomentEffect::WisdomMultiplier => format!("2x Wisdom ({:.0}s)", remaining),
        MomentEffect::ClickFrenzy => format!("3x Clicks ({:.0}s)", remaining),
        _ => return,
    };

    commands.spawn((
        Text::new(label),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(buff.effect.color()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-80.0)),
            ..default()
        },
        BuffIndicator,
    ));
}
