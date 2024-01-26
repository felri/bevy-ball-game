use super::DebriUniverse;
use bevy::prelude::*;

pub fn insert_debri_universe(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();

    commands.insert_resource(DebriUniverse::new(
        Vec2::new(0.0, 0.0),
        Vec2::new(window.width(), window.height()),
    ));
}
