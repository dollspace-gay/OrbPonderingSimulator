use crate::gameplay::wisdom::TruthGenerated;
use bevy::prelude::*;

#[derive(Component)]
pub struct TruthPopup {
    pub lifetime: Timer,
}

pub fn show_truth_popup(mut commands: Commands, mut truth_events: MessageReader<TruthGenerated>) {
    for event in truth_events.read() {
        commands.spawn((
            Text::new(format!("\"{}\"", event.text)),
            TextFont {
                font_size: 26.0,
                ..default()
            },
            TextColor(Color::srgba(1.0, 0.95, 0.7, 1.0)),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(35.0),
                left: Val::Percent(10.0),
                right: Val::Percent(10.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            TruthPopup {
                lifetime: Timer::from_seconds(6.0, TimerMode::Once),
            },
        ));
    }
}

pub fn animate_truth_popup(
    mut commands: Commands,
    time: Res<Time>,
    mut popups: Query<(Entity, &mut TruthPopup, &mut TextColor)>,
) {
    for (entity, mut popup, mut color) in &mut popups {
        popup.lifetime.tick(time.delta());

        let remaining = popup.lifetime.remaining_secs();
        if remaining < 2.0 {
            let alpha = remaining / 2.0;
            color.0 = Color::srgba(1.0, 0.95, 0.7, alpha);
        }

        if popup.lifetime.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
