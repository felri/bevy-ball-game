use bevy::prelude::*;

#[derive(Component)]
pub struct Building;

#[derive(Event)]
pub struct EventSpawnBuilding {
    pub position: Transform,
}

pub enum BuldingType {
    Stash,
    Shooter,
}
