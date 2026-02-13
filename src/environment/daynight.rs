use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct DayNightCycle {
    pub time_of_day: f32,
    pub cycle_speed: f32,
}

impl Default for DayNightCycle {
    fn default() -> Self {
        Self {
            time_of_day: 0.8,
            cycle_speed: 0.01,
        }
    }
}

pub fn update_cycle(mut cycle: ResMut<DayNightCycle>, time: Res<Time>) {
    cycle.time_of_day = (cycle.time_of_day + cycle.cycle_speed * time.delta_secs()) % 1.0;
}
