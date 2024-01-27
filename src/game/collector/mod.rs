pub mod components;
mod systems;

use self::components::CollectorSpawnEvent;

use super::{debri::PHYISCS_TICK_RATE, SimulationState};
use crate::AppState;

use bevy::{prelude::*, time::common_conditions::on_timer};
use instant::Duration;
use systems::*;

pub const COLLECTOR_SIZE: f32 = 10.0;

pub struct CollectorPlugin;

impl Plugin for CollectorPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<CollectorSpawnEvent>()
            // Systems
            .add_systems(
                Update,
                (collector_movement, spawn_collector)
                    .run_if(on_timer(Duration::from_secs_f32(1. / PHYISCS_TICK_RATE)))
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_collector);
    }
}
