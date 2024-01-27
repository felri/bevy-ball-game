use bevy::prelude::*;

#[derive(Component)]
pub struct Building;

#[derive(Event)]
pub struct EvenetSpawnBuilding {
    pub position: Vec3,
}
