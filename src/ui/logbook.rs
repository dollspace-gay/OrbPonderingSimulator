use crate::gameplay::{state::GameState, wisdom::TruthGenerated};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct LogbookEntry {
    pub text: String,
    pub truth_number: u32,
}

#[derive(Resource, Default)]
pub struct Logbook {
    pub entries: Vec<LogbookEntry>,
}

#[derive(Component)]
pub struct LogbookPanel;

pub fn record_truths(
    mut logbook: ResMut<Logbook>,
    mut truth_events: MessageReader<TruthGenerated>,
) {
    for event in truth_events.read() {
        let num = logbook.entries.len() as u32 + 1;
        logbook.entries.push(LogbookEntry {
            text: event.text.clone(),
            truth_number: num,
        });
    }
}

pub fn toggle_logbook(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyL) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::LogbookOpen),
            GameState::LogbookOpen => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

pub fn open_logbook(mut commands: Commands, logbook: Res<Logbook>) {
    // Semi-transparent backdrop + scrollable panel
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            LogbookPanel,
        ))
        .with_children(|backdrop| {
            // Panel container
            backdrop
                .spawn((
                    Node {
                        width: Val::Px(600.0),
                        max_height: Val::Percent(80.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(24.0)),
                        row_gap: Val::Px(16.0),
                        overflow: Overflow::scroll_y(),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.06, 0.14, 0.95)),
                ))
                .with_children(|panel| {
                    // Title
                    panel.spawn((
                        Text::new("Logbook of Pondered Truths"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.85, 0.4)),
                    ));

                    // Divider
                    panel.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(1.0, 0.85, 0.4, 0.3)),
                    ));

                    if logbook.entries.is_empty() {
                        panel.spawn((
                            Text::new("No truths pondered yet.\nHold [SPACE] to ponder the orb..."),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgba(0.6, 0.55, 0.7, 0.7)),
                        ));
                    } else {
                        // Entries in reverse order (newest first)
                        for entry in logbook.entries.iter().rev() {
                            panel
                                .spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(4.0),
                                    padding: UiRect::vertical(Val::Px(6.0)),
                                    ..default()
                                })
                                .with_children(|row| {
                                    row.spawn((
                                        Text::new(format!("Truth #{}", entry.truth_number)),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgba(1.0, 0.8, 0.3, 0.6)),
                                    ));
                                    row.spawn((
                                        Text::new(format!("\"{}\"", entry.text)),
                                        TextFont {
                                            font_size: 20.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.9, 0.88, 0.8)),
                                    ));
                                });
                        }
                    }

                    // Footer hint
                    panel.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            margin: UiRect::top(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(1.0, 0.85, 0.4, 0.3)),
                    ));
                    panel.spawn((
                        Text::new("Press [L] to close"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.6, 0.55, 0.7, 0.5)),
                    ));
                });
        });
}

pub fn close_logbook(mut commands: Commands, panels: Query<Entity, With<LogbookPanel>>) {
    for entity in &panels {
        commands.entity(entity).despawn();
    }
}
