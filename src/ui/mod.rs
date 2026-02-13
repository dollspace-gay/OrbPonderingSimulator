use crate::gameplay::state::GameState;
use bevy::prelude::*;

pub mod hud;
pub mod logbook;
pub mod truth_display;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<logbook::Logbook>()
            .add_systems(Startup, hud::setup_hud)
            .add_systems(
                Update,
                (
                    hud::update_wisdom_display,
                    hud::update_afp_display,
                    hud::update_acolyte_display,
                    hud::update_generator_display,
                    hud::update_deep_focus_display,
                    hud::update_secondary_display,
                    truth_display::show_truth_popup,
                    truth_display::animate_truth_popup,
                    logbook::record_truths,
                ),
            )
            .add_systems(Update, logbook::toggle_logbook)
            .add_systems(OnEnter(GameState::LogbookOpen), logbook::open_logbook)
            .add_systems(OnExit(GameState::LogbookOpen), logbook::close_logbook);
    }
}
