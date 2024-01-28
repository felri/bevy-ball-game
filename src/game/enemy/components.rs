use bevy::prelude::*;

#[derive(Component)]

pub struct Enemy;

#[derive(Event)]
pub struct EnemySpawnEvent {
    pub spawn_pos: Transform,
}
