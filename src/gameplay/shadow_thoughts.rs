use super::wisdom::WisdomMeter;
use bevy::prelude::*;
use rand::Rng;

/// Shadow Thoughts: dark wisps that attach to the orb, draining visible wisdom
/// but secretly storing it. When dispelled, stored wisdom returns multiplied.
#[derive(Resource)]
pub struct ShadowState {
    /// Currently attached shadow thoughts
    pub count: u32,
    /// Maximum shadows that can attach at once
    pub max_shadows: u32,
    /// Timer until next shadow spawns
    pub spawn_timer: Timer,
    /// Wisdom secretly stored by shadows (before multiplier)
    pub stored_wisdom: f64,
    /// Drain rate per shadow (0.1 = 10% of generation)
    pub drain_per_shadow: f32,
    /// Multiplier per shadow when dispelled (1.1 = +10% per shadow)
    pub dispel_multiplier: f64,
}

impl Default for ShadowState {
    fn default() -> Self {
        let delay = rand::thread_rng().gen_range(120.0..300.0);
        Self {
            count: 0,
            max_shadows: 5,
            spawn_timer: Timer::from_seconds(delay, TimerMode::Once),
            stored_wisdom: 0.0,
            drain_per_shadow: 0.10,
            dispel_multiplier: 1.1,
        }
    }
}

impl ShadowState {
    /// Fraction of wisdom being drained (0.0 to max_shadows * drain_per_shadow)
    pub fn drain_fraction(&self) -> f32 {
        (self.count as f32 * self.drain_per_shadow).min(0.95)
    }

    /// The multiplier applied to stored wisdom when all shadows are dispelled
    pub fn total_dispel_multiplier(&self) -> f64 {
        self.dispel_multiplier.powi(self.count as i32)
    }

    fn reset_spawn_timer(&mut self) {
        let delay = rand::thread_rng().gen_range(90.0..240.0);
        self.spawn_timer = Timer::from_seconds(delay, TimerMode::Once);
    }
}

// ========== SYSTEMS ==========

/// Spawns new shadow thoughts over time
pub fn update_shadows(mut shadows: ResMut<ShadowState>, time: Res<Time>) {
    if shadows.count >= shadows.max_shadows {
        return;
    }

    shadows.spawn_timer.tick(time.delta());
    if shadows.spawn_timer.just_finished() {
        shadows.count += 1;
        shadows.reset_spawn_timer();
    }
}

/// Siphons a portion of wisdom generation into shadow storage.
/// Runs every frame: calculates what was added this frame and redirects a fraction.
pub fn siphon_wisdom(
    mut shadows: ResMut<ShadowState>,
    mut wisdom: ResMut<WisdomMeter>,
    mut last_wisdom: Local<f32>,
) {
    if shadows.count == 0 {
        *last_wisdom = wisdom.current;
        return;
    }

    // How much wisdom was added since last frame
    let gained = wisdom.current - *last_wisdom;
    if gained <= 0.0 {
        *last_wisdom = wisdom.current;
        return;
    }

    let drain = gained * shadows.drain_fraction();
    wisdom.current -= drain;
    shadows.stored_wisdom += drain as f64;
    *last_wisdom = wisdom.current;
}

/// Player presses [D] to dispel all shadows and reclaim stored wisdom with multiplier
pub fn handle_dispel(
    keys: Res<ButtonInput<KeyCode>>,
    mut shadows: ResMut<ShadowState>,
    mut wisdom: ResMut<WisdomMeter>,
) {
    if !keys.just_pressed(KeyCode::KeyD) {
        return;
    }

    if shadows.count == 0 {
        return;
    }

    let multiplied = shadows.stored_wisdom * shadows.total_dispel_multiplier();
    wisdom.current += multiplied as f32;

    shadows.count = 0;
    shadows.stored_wisdom = 0.0;
    shadows.reset_spawn_timer();
}

// ========== UI ==========

#[derive(Component)]
pub struct ShadowIndicator;

/// Renders shadow thought indicators when shadows are active
pub fn render_shadow_ui(
    mut commands: Commands,
    shadows: Res<ShadowState>,
    existing: Query<Entity, With<ShadowIndicator>>,
) {
    if !shadows.is_changed() {
        return;
    }

    for entity in &existing {
        commands.entity(entity).despawn();
    }

    if shadows.count == 0 {
        return;
    }

    let drain_pct = shadows.drain_fraction() * 100.0;
    let stored = shadows.stored_wisdom;
    let multiplier = shadows.total_dispel_multiplier();
    let payout = stored * multiplier;

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(70.0),
                left: Val::Px(16.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(3.0),
                padding: UiRect::all(Val::Px(10.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.0, 0.1, 0.8)),
            ShadowIndicator,
        ))
        .with_children(|panel| {
            // Shadow wisp icons
            let wisps: String = (0..shadows.count).map(|_| "\u{25CF} ").collect();
            panel.spawn((
                Text::new(format!("Shadow Thoughts {}", wisps.trim())),
                TextFont { font_size: 15.0, ..default() },
                TextColor(Color::srgb(0.5, 0.2, 0.6)),
            ));

            // Drain info
            panel.spawn((
                Text::new(format!("-{:.0}% wisdom rate", drain_pct)),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgba(0.8, 0.3, 0.3, 0.8)),
            ));

            // Stored wisdom
            if stored > 0.1 {
                panel.spawn((
                    Text::new(format!(
                        "Stored: {:.1} (x{:.2} = {:.1})",
                        stored, multiplier, payout
                    )),
                    TextFont { font_size: 13.0, ..default() },
                    TextColor(Color::srgba(0.6, 0.4, 0.9, 0.7)),
                ));
            }

            // Dispel hint
            panel.spawn((
                Text::new("[D] Dispel shadows"),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgba(0.5, 0.5, 0.6, 0.5)),
            ));
        });
}
