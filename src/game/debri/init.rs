use super::DebriUniverse;
use bevy::prelude::*;

pub fn inser_debri_universe(
    mut commands: Commands,
    window: Query<&Window>,
    assets: Res<AssetServer>,
) {
    let window = window.single();

    commands.insert_resource(DebriUniverse::new(
        Vec2::new(window.width() / -2.0, window.height() / -2.0),
        Vec2::new(window.width() / 2.0, window.height() / 2.0),
    ));
}
