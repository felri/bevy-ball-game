pub mod components;
mod init;
mod systems;

use init::*;
use std::time::Duration;
pub mod resources;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;
use bevy::time::common_conditions::on_timer;

use bevy::prelude::*;

pub const DEBRI_SIZE: f32 = 8.0;
pub const PHYISCS_TICK_RATE: f32 = 90.;

pub struct DebriPlugin;

impl Plugin for DebriPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(QuadBench::default())
            .add_event::<components::SpawnDebri>()
            .add_event::<components::CollectedEvent>()
            // Systems
            .add_systems(Startup, insert_debri_universe)
            .add_systems(
                FixedUpdate,
                (spawn_debri, handle_debri_collected_event, count_debri)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (build_or_update_quadtree, update_debri, move_system)
                    .run_if(on_timer(Duration::from_secs_f32(1. / PHYISCS_TICK_RATE)))
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_debri);
    }
}
