use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use super::components::Player;

use crate::game::components::OrbitCenter;
use crate::game::components::Velocity;

pub fn orbit_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, &OrbitCenter), With<OrbitCenter>>,
) {
    for (mut position, velocity, orbit_center) in query.iter_mut() {
        let delta_time = time.delta_seconds();

        // Calculate the new position
        let dx = position.translation.x - orbit_center.x;
        let dy = position.translation.y - orbit_center.y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Update the angle based on the velocity
        let velocity_magnitude = velocity.value.length() * 0.5;
        let new_angle = f32::atan2(dy, dx) + velocity_magnitude * delta_time;

        position.translation = Vec3::new(
            orbit_center.x + distance * f32::cos(new_angle),
            orbit_center.y + distance * f32::sin(new_angle),
            0.0,
        );
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let initial_speed = rand::random::<f32>() * 0.5;
    let velocity = Vec3::new(
        (rand::random::<f32>() - 0.5) * initial_speed,
        (rand::random::<f32>() - 0.5) * initial_speed,
        0.0,
    );

    // random position in a circle of the radius of the window
    let position = Transform::from_xyz(
        window.width() / 2.0 + (rand::random::<f32>() - 0.5) * window.width(),
        window.height() / 2.0 + (rand::random::<f32>() - 0.5) * window.height(),
        0.0,
    );

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(position.translation.x, position.translation.y, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player,
        Velocity {
            value: velocity,
            damping: 2.0,
            min_speed: 50.0,
        },
        OrbitCenter {
            x: window.width() / 2.0,
            y: window.height() / 2.0,
        },
    ));

    let position = Transform::from_xyz(
        window.width() / 2.0 + (rand::random::<f32>() - 0.5) * window.width(),
        window.height() / 2.0 + (rand::random::<f32>() - 0.5) * window.height(),
        0.0,
    );
    // at the top
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(position.translation.x, position.translation.y, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player,
        Velocity {
            value: velocity,
            damping: 2.0,
            min_speed: 50.0,
        },
        OrbitCenter {
            x: window.width() / 2.0,
            y: window.height() / 2.0,
        },
    ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}
