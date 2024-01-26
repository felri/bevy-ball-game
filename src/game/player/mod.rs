pub mod components;
mod systems;

use systems::*;

use super::SimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter State
            .add_systems(OnEnter(AppState::Game), spawn_player)
            // Systems
            .add_systems(
                FixedUpdate,
                (orbit_system,)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
