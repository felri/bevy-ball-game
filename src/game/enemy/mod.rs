pub mod components;
mod systems;

use self::components::EnemySpawnEvent;

use super::{debri::PHYISCS_TICK_RATE, SimulationState};
use crate::AppState;

use bevy::{prelude::*, time::common_conditions::on_timer};
use instant::Duration;
use systems::*;

pub const ENEMY_SIZE: f32 = 10.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<EnemySpawnEvent>()
            // Systems
            .add_systems(
                FixedUpdate,
                (enemy_movement, spawn_enemy)
                    .run_if(on_timer(Duration::from_secs_f32(1. / PHYISCS_TICK_RATE)))
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_enemy);
    }
}
