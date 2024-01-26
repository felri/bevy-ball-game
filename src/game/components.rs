use bevy::prelude::*;

#[derive(Component, Clone, Copy, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
#[derive(Component, Clone)]
pub struct Velocity {
    pub value: Vec3,
}
#[derive(Component)]
pub struct OrbitCenter {
    pub x: f32,
    pub y: f32,
}
