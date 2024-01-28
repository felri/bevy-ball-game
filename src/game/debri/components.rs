use bevy::prelude::*;

use crate::game::components::Position;
use crate::quadtree::{coord::Coord, region::Region, slot_map::SlotId};

#[derive(Component, Clone)]

pub struct Debri;

#[derive(Event)]
pub struct SpawnDebri {
    pub direction: Vec2,
    pub position: Position,
}

#[derive(Component)]
pub struct Collected;

#[derive(Component, Debug)]
pub struct Collider {
    pub id: Option<SlotId>,
    pub radius: f32,
    pub nearby: usize,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            id: None,
            radius,
            nearby: 0,
        }
    }
    pub fn into_region(&self, origin: Vec3) -> Region {
        let min =
            Coord::from_f32(origin.x, origin.y) - Coord::from_f32(self.radius, self.radius) / 2;
        let max =
            Coord::from_f32(origin.x, origin.y) + Coord::from_f32(self.radius, self.radius) / 2;

        Region::new(min, max)
    }
}

#[derive(Debug)]
pub struct Body {
    pub entity: Entity,
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct QuadNodeRect;

#[derive(Event)]
pub struct CollectedEvent {
    pub entity: Entity,
}
