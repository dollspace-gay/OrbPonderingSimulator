use bevy::prelude::*;

pub mod achievements;
pub mod acolytes;
pub mod generators;
pub mod moments;
pub mod pondering;
pub mod progression;
pub mod schools;
pub mod shop;
pub mod state;
pub mod synergies;
pub mod transcendence;
pub mod wisdom;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<state::GameState>()
            .init_resource::<pondering::PonderState>()
            .init_resource::<wisdom::WisdomMeter>()
            .init_resource::<progression::ArcaneProgress>()
            .init_resource::<acolytes::AcolyteState>()
            .init_resource::<generators::GeneratorState>()
            .init_resource::<synergies::SynergyState>()
            .init_resource::<schools::SchoolState>()
            .init_resource::<moments::MomentState>()
            .init_resource::<transcendence::TranscendenceState>()
            .init_resource::<achievements::AchievementTracker>()
            .add_message::<wisdom::TruthGenerated>()
            .add_systems(
                Update,
                (
                    pondering::handle_click_ponder,
                    pondering::handle_deep_focus,
                    pondering::update_ponder_visuals,
                    acolytes::passive_wisdom,
                    synergies::recalculate_synergies,
                    generators::passive_generator_wisdom,
                    moments::update_moments,
                    moments::handle_moment_click,
                    moments::render_moment_popup,
                    moments::render_buff_indicator,
                )
                    .run_if(in_state(state::GameState::Playing)),
            )
            .add_systems(
                Update,
                acolytes::summon_acolyte.run_if(in_state(state::GameState::Playing)),
            )
            .add_systems(
                Update,
                wisdom::check_truth_generation.run_if(in_state(state::GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    progression::award_points,
                    transcendence::accumulate_run_wisdom,
                    schools::track_run_truths,
                    achievements::track_achievement_stats,
                    achievements::track_deep_focus_uses,
                    achievements::check_achievements,
                )
                    .run_if(in_state(state::GameState::Playing)),
            )
            // School selection
            .add_systems(
                OnEnter(state::GameState::SchoolSelection),
                schools::open_school_selection,
            )
            .add_systems(
                OnExit(state::GameState::SchoolSelection),
                schools::close_school_selection,
            )
            .add_systems(
                Update,
                schools::handle_school_choice
                    .run_if(in_state(state::GameState::SchoolSelection)),
            )
            // Transcendence
            .add_systems(Update, transcendence::toggle_transcendence)
            .add_systems(
                OnEnter(state::GameState::TranscendenceOpen),
                transcendence::open_transcendence_ui,
            )
            .add_systems(
                OnExit(state::GameState::TranscendenceOpen),
                transcendence::close_transcendence_ui,
            )
            .add_systems(
                Update,
                (
                    transcendence::handle_transcend_click,
                    transcendence::handle_enlightenment_buy,
                )
                    .run_if(in_state(state::GameState::TranscendenceOpen)),
            )
            // Achievements
            .add_systems(Update, achievements::toggle_achievements)
            .add_systems(
                Update,
                (
                    achievements::spawn_notifications,
                    achievements::update_notifications,
                ),
            )
            .add_systems(
                OnEnter(state::GameState::AchievementsOpen),
                achievements::open_achievements,
            )
            .add_systems(
                OnExit(state::GameState::AchievementsOpen),
                achievements::close_achievements,
            )
            // Pause
            .add_systems(Update, state::toggle_pause)
            .add_systems(OnEnter(state::GameState::Paused), state::show_pause_overlay)
            .add_systems(OnExit(state::GameState::Paused), state::hide_pause_overlay)
            // Shop
            .init_resource::<shop::ShopCatalog>()
            .init_resource::<shop::PurchaseTracker>()
            .add_systems(Update, shop::toggle_shop)
            .add_systems(OnEnter(state::GameState::ShopOpen), shop::open_shop)
            .add_systems(OnExit(state::GameState::ShopOpen), shop::close_shop)
            .add_systems(
                Update,
                (
                    shop::handle_category_click,
                    shop::handle_buy_click,
                    shop::handle_buy_generator,
                    shop::handle_equip_click,
                    shop::rebuild_item_list,
                    shop::update_tab_backgrounds,
                    shop::update_shop_buttons,
                    shop::update_shop_afp,
                )
                    .run_if(in_state(state::GameState::ShopOpen)),
            );
    }
}
