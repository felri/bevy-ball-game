use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::game::components::Position;
use crate::game::debri::components::SpawnDebri;
use crate::game::player::components::Player;
use crate::game::target::components::Target;

use super::components::Projectile;
use super::resources::*;

pub fn projectile_movement(
    mut projectile_query: Query<(&mut Transform, &Projectile)>,
    time: Res<Time>,
) {
    projectile_query
        .par_iter_mut()
        .for_each(|(mut transform, projectile)| {
            let direction = projectile.target - transform.translation;
            let velocity = direction.normalize() * time.delta_seconds() * 200.0;
            transform.translation += velocity;
        });
}

pub fn despawn_projectile(
    mut commands: Commands,
    projectile_query: Query<Entity, With<Projectile>>,
) {
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn tick_projectile_spawn_timer(
    mut projectile_spawn_timer: ResMut<ProjectileSpawnTimer>,
    time: Res<Time>,
) {
    projectile_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_projectile_timer(
    mut commands: Commands,
    projectile_spawn_timer: Res<ProjectileSpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
    target_query: Query<&Transform, With<Target>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if projectile_spawn_timer.timer.finished() {
        let target_transform = target_query.get_single().unwrap();
        for player_transform in player_query.iter() {
            let dx = target_transform.translation.x - player_transform.translation.x;
            let dy = target_transform.translation.y - player_transform.translation.y;
            let rotation = dy.atan2(dx);

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad::new(Vec2::new(12.0, 2.0))))
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform {
                        translation: Vec3::new(
                            player_transform.translation.x,
                            player_transform.translation.y,
                            0.0,
                        ),
                        rotation: Quat::from_rotation_z(rotation),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Projectile {
                    target: target_transform.translation,
                },
            ));
        }
    }
}

pub fn projectile_hit_target(
    mut commands: Commands,
    mut events_writer: EventWriter<SpawnDebri>,
    mut projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    target_query: Query<&Transform, With<Target>>,
) {
    for (entity, projectile_transform) in projectile_query.iter_mut() {
        if let Ok(target_transform) = target_query.get_single() {
            let distance = projectile_transform
                .translation
                .distance(target_transform.translation);
            if distance < 10.0 {
                // Direction from target to projectile
                let direction = projectile_transform.translation - target_transform.translation;
                let direction = Vec2::new(direction.x, direction.y).normalize();
                let position = Position {
                    x: target_transform.translation.x,
                    y: target_transform.translation.y,
                };

                // Spawn debris
                events_writer.send(SpawnDebri {
                    position,
                    direction,
                });

                // Despawn projectile
                commands.entity(entity).despawn();
            }
        }
    }
}
