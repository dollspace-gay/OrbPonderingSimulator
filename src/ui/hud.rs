use crate::gameplay::{
    acolytes::AcolyteState, generators::GeneratorState, pondering::PonderState,
    progression::ArcaneProgress, shop::PurchaseTracker, synergies::SynergyState,
    wisdom::WisdomMeter,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct WisdomText;

#[derive(Component)]
pub struct AfpText;

#[derive(Component)]
pub struct WisdomBar;

#[derive(Component)]
pub struct PonderHint;

#[derive(Component)]
pub struct AcolyteText;

#[derive(Component)]
pub struct SummonCostText;

#[derive(Component)]
pub struct GeneratorText;

#[derive(Component)]
pub struct DeepFocusText;

pub fn setup_hud(mut commands: Commands) {
    // Root
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::SpaceBetween,
        ..default()
    }).with_children(|parent| {
        // Top bar
        parent.spawn(Node {
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(16.0)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexStart,
            ..default()
        }).with_children(|top| {
            // Left: wisdom meter + Deep Focus
            top.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            }).with_children(|left| {
                left.spawn((
                    Text::new("Wisdom: 0%"),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.7, 1.0)),
                    WisdomText,
                ));

                // Bar background
                left.spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.3, 0.2, 0.5, 0.5)),
                )).with_children(|bg| {
                    bg.spawn((
                        Node {
                            width: Val::Percent(0.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.6, 0.4, 1.0)),
                        WisdomBar,
                    ));
                });

                // Deep Focus indicator
                left.spawn((
                    Text::new("Deep Focus: READY"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.4, 0.8, 1.0)),
                    DeepFocusText,
                ));
            });

            // Right: AFP + Acolytes
            top.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                row_gap: Val::Px(4.0),
                ..default()
            }).with_children(|right| {
                right.spawn((
                    Text::new("Arcane Focus: 0"),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::srgb(1.0, 0.85, 0.3)),
                    AfpText,
                ));

                right.spawn((
                    Text::new("Acolytes: 0"),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::srgb(0.6, 0.9, 0.7)),
                    AcolyteText,
                ));

                right.spawn((
                    Text::new("[A] Summon (20 AFP)"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.5, 0.7, 0.5)),
                    SummonCostText,
                ));

                right.spawn((
                    Text::new(""),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.7, 0.6, 0.9)),
                    GeneratorText,
                ));
            });
        });

        // Bottom hint
        parent.spawn(Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            padding: UiRect::bottom(Val::Px(32.0)),
            ..default()
        }).with_children(|bottom| {
            bottom.spawn((
                Text::new("[Click] Ponder | [SPACE] Deep Focus | [A] Summon | [D] Dispel | [F] Pet | [B] Shop | [L] Logbook | [T] Transcend | [V] Achievements | [C] Challenges"),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::srgba(0.6, 0.6, 0.7, 0.6)),
                PonderHint,
            ));
        });
    });
}

pub fn update_wisdom_display(
    wisdom: Res<WisdomMeter>,
    mut text_query: Query<&mut Text, With<WisdomText>>,
    mut bar_query: Query<&mut Node, With<WisdomBar>>,
) {
    let pct = (wisdom.current / wisdom.max_wisdom * 100.0).min(100.0);
    for mut text in &mut text_query {
        **text = format!("Wisdom: {:.0}%", pct);
    }
    for mut node in &mut bar_query {
        node.width = Val::Percent(pct);
    }
}

pub fn update_afp_display(
    progress: Res<ArcaneProgress>,
    mut text_query: Query<&mut Text, With<AfpText>>,
) {
    for mut text in &mut text_query {
        **text = format!("Arcane Focus: {}", format_afp(progress.focus_points));
    }
}

pub fn update_generator_display(
    generators: Res<GeneratorState>,
    synergies: Res<SynergyState>,
    tracker: Res<PurchaseTracker>,
    mut text_query: Query<&mut Text, With<GeneratorText>>,
) {
    let base = synergies.total_synergized_production(&generators);
    if base <= 0.0 {
        for mut text in &mut text_query {
            **text = String::new();
        }
        return;
    }
    let rate = base * (1.0 + tracker.efficiency_bonus as f64) * tracker.wisdom_speed_bonus as f64;
    for mut text in &mut text_query {
        **text = format!("Generators: +{:.1}/s", rate);
    }
}

fn format_afp(value: u64) -> String {
    if value >= 1_000_000_000 {
        format!("{:.1}B", value as f64 / 1_000_000_000.0)
    } else if value >= 1_000_000 {
        format!("{:.1}M", value as f64 / 1_000_000.0)
    } else if value >= 10_000 {
        format!("{:.1}K", value as f64 / 1_000.0)
    } else {
        format!("{}", value)
    }
}

pub fn update_acolyte_display(
    acolytes: Res<AcolyteState>,
    tracker: Res<PurchaseTracker>,
    mut acolyte_text: Query<&mut Text, With<AcolyteText>>,
    mut cost_text: Query<&mut Text, (With<SummonCostText>, Without<AcolyteText>)>,
    mut cost_color: Query<&mut TextColor, With<SummonCostText>>,
    progress: Res<ArcaneProgress>,
) {
    let rate =
        acolytes.passive_rate() * (1.0 + tracker.efficiency_bonus) * tracker.wisdom_speed_bonus;

    for mut text in &mut acolyte_text {
        if acolytes.count > 0 {
            **text = format!("Acolytes: {} (+{:.1}/s)", acolytes.count, rate);
        } else {
            **text = "Acolytes: 0".to_string();
        }
    }

    let cost = acolytes.next_cost();
    for mut text in &mut cost_text {
        **text = format!("[A] Summon ({} AFP)", cost);
    }

    let can_afford = progress.focus_points >= cost;
    for mut color in &mut cost_color {
        color.0 = if can_afford {
            Color::srgb(0.5, 1.0, 0.5)
        } else {
            Color::srgb(0.5, 0.4, 0.4)
        };
    }
}

pub fn update_deep_focus_display(
    ponder: Res<PonderState>,
    mut text_query: Query<&mut Text, With<DeepFocusText>>,
    mut color_query: Query<&mut TextColor, With<DeepFocusText>>,
) {
    for mut text in &mut text_query {
        if ponder.deep_focus_active {
            **text = format!("Deep Focus: Active ({:.0}s)", ponder.deep_focus_timer);
        } else if ponder.deep_focus_cooldown > 0.0 {
            **text = format!("Deep Focus: Cooldown ({:.0}s)", ponder.deep_focus_cooldown);
        } else {
            **text = "Deep Focus: READY".to_string();
        }
    }

    for mut color in &mut color_query {
        color.0 = if ponder.deep_focus_active {
            Color::srgb(0.3, 1.0, 1.0)
        } else if ponder.deep_focus_cooldown > 0.0 {
            Color::srgb(0.5, 0.5, 0.6)
        } else {
            Color::srgb(0.4, 0.8, 1.0)
        };
    }
}
