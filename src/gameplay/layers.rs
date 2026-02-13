use super::transcendence::TranscendenceState;
use super::wisdom::{TruthGenerated, WisdomMeter};
use crate::environment::daynight::DayNightCycle;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ========== CONTENT LAYERS ==========

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentLayer {
    Surface,
    Astral,
    Dream,
    Void,
}

impl ContentLayer {
    pub const ALL: [ContentLayer; 4] = [Self::Surface, Self::Astral, Self::Dream, Self::Void];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Surface => "Surface Plane",
            Self::Astral => "Astral Plane",
            Self::Dream => "Dream Plane",
            Self::Void => "Void Plane",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Surface => "The familiar realm of waking thought.",
            Self::Astral => "Night amplifies passive wisdom generation.",
            Self::Dream => "Dream truths manifest at night. Wisdom flows stronger under the moon.",
            Self::Void => "The space between spaces. Its secrets remain hidden... for now.",
        }
    }

    pub fn required_transcendences(&self) -> u32 {
        match self {
            Self::Surface => 0,
            Self::Astral => 1,
            Self::Dream => 5,
            Self::Void => 20,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Surface => Color::srgb(0.6, 0.7, 0.6),
            Self::Astral => Color::srgb(0.4, 0.6, 1.0),
            Self::Dream => Color::srgb(0.7, 0.4, 0.9),
            Self::Void => Color::srgb(0.3, 0.1, 0.2),
        }
    }
}

// ========== LAYER STATE ==========

#[derive(Resource, Debug)]
pub struct LayerState {
    pub unlocked: Vec<ContentLayer>,
    pub notification_queue: Vec<ContentLayer>,
}

impl Default for LayerState {
    fn default() -> Self {
        Self {
            unlocked: vec![ContentLayer::Surface],
            notification_queue: Vec::new(),
        }
    }
}

impl LayerState {
    pub fn has(&self, layer: ContentLayer) -> bool {
        self.unlocked.contains(&layer)
    }

    pub fn highest_unlocked(&self) -> ContentLayer {
        // Return highest layer (last in ALL order)
        for &layer in ContentLayer::ALL.iter().rev() {
            if self.has(layer) {
                return layer;
            }
        }
        ContentLayer::Surface
    }

    pub fn unlock(&mut self, layer: ContentLayer) {
        if !self.has(layer) {
            self.unlocked.push(layer);
            self.notification_queue.push(layer);
        }
    }

    /// Night factor: 0.0 at day peak, 1.0 at night peak
    pub fn night_factor(cycle: &DayNightCycle) -> f32 {
        let t = (cycle.time_of_day * std::f32::consts::TAU).sin() * 0.5 + 0.5;
        1.0 - t
    }

    /// Dream layer wisdom multiplier (up to 1.5x at peak night)
    pub fn dream_multiplier(&self, cycle: &DayNightCycle) -> f32 {
        if !self.has(ContentLayer::Dream) {
            return 1.0;
        }
        let nf = Self::night_factor(cycle);
        1.0 + nf * 0.5
    }
}

// ========== DREAM TRUTHS ==========

const DREAM_TRUTHS: &[&str] = &[
    "In the dream, the orb spoke your name backwards.",
    "The sleeping mind walks paths the waking mind cannot see.",
    "Between one dream and the next, a truth slipped through.",
    "The moon whispered a secret the sun would never tell.",
    "In the dream realm, every thought has weight and color.",
    "You dreamed of an orb dreaming of you.",
    "The night sky opened like a book written in starlight.",
    "A truth arrived wrapped in sleep, addressed to no one.",
    "The dream orb pulses with truths that dissolve at dawn.",
    "In the space between waking and sleep, wisdom accumulates like dew.",
];

#[derive(Resource)]
pub struct DreamTruthTimer(pub Timer);

impl Default for DreamTruthTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(45.0, TimerMode::Repeating))
    }
}

// ========== SYSTEMS ==========

/// Check transcendence count and unlock layers
pub fn check_layer_unlocks(
    transcendence: Res<TranscendenceState>,
    mut layers: ResMut<LayerState>,
) {
    for &layer in &ContentLayer::ALL {
        if transcendence.total_transcendences >= layer.required_transcendences()
            && !layers.has(layer)
        {
            layers.unlock(layer);
        }
    }
}

/// Astral layer: flat passive wisdom bonus scaled by night
pub fn apply_astral_bonus(
    layers: Res<LayerState>,
    cycle: Res<DayNightCycle>,
    mut wisdom: ResMut<WisdomMeter>,
    time: Res<Time>,
) {
    if !layers.has(ContentLayer::Astral) {
        return;
    }
    let nf = LayerState::night_factor(&cycle);
    // 0.05/s during day, up to 0.5/s at peak night
    let rate = 0.05 + nf * 0.45;
    wisdom.current += rate * time.delta_secs();
}

/// Dream layer: periodic dream truths at night
pub fn dream_truth_generation(
    layers: Res<LayerState>,
    cycle: Res<DayNightCycle>,
    mut timer: ResMut<DreamTruthTimer>,
    mut wisdom: ResMut<WisdomMeter>,
    mut truth_messages: MessageWriter<TruthGenerated>,
    time: Res<Time>,
) {
    if !layers.has(ContentLayer::Dream) {
        return;
    }

    let nf = LayerState::night_factor(&cycle);
    if nf < 0.6 {
        return; // Only at night
    }

    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        // Grant bonus wisdom (equivalent to ~10 truths worth)
        wisdom.current += 10.0;

        // Pick a random dream truth
        let index = (wisdom.truths_generated as usize) % DREAM_TRUTHS.len();
        truth_messages.write(TruthGenerated {
            text: DREAM_TRUTHS[index].to_string(),
            truth_index: usize::MAX, // Sentinel: not a codex truth
        });
    }
}

// ========== NOTIFICATIONS ==========

#[derive(Component)]
pub struct LayerNotification {
    pub timer: f32,
}

pub fn spawn_layer_notifications(
    mut commands: Commands,
    mut layers: ResMut<LayerState>,
) {
    while let Some(layer) = layers.notification_queue.pop() {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(80.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                LayerNotification { timer: 5.0 },
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        },
                        BackgroundColor(layer.color().with_alpha(0.9)),
                    ))
                    .with_children(|badge| {
                        badge.spawn((
                            Text::new(format!("Layer Unlocked: {}", layer.name())),
                            TextFont { font_size: 20.0, ..default() },
                            TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        ));
                        badge.spawn((
                            Text::new(layer.description()),
                            TextFont { font_size: 13.0, ..default() },
                            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                        ));
                    });
            });
    }
}

pub fn update_layer_notifications(
    mut commands: Commands,
    mut query: Query<(Entity, &mut LayerNotification)>,
    time: Res<Time>,
) {
    for (entity, mut notif) in &mut query {
        notif.timer -= time.delta_secs();
        if notif.timer <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
