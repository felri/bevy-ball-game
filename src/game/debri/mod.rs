pub mod components;
mod init;
mod systems;

use init::*;
use std::time::Duration;
mod resources;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

use bevy::prelude::*;

pub const DEBRI_SIZE: f32 = 3.0;
pub const PHYISCS_TICK_RATE: f32 = 90.;
pub const BOID_SPAWN_RATE: f32 = 100.0;
pub const CURSOR_QUAD_SIZE: f32 = 100.0;
pub const BOID_SIZE: f32 = 5.0;

pub struct DebriPlugin;

impl Plugin for DebriPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(QuadBench::default())
            .add_event::<components::SpawnDebri>()
            // Systems
            .add_systems(Startup, inser_debri_universe)
            .add_systems(
                FixedUpdate,
                (spawn_debri)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_debri);
    }
}
