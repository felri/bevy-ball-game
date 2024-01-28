use crate::game::{
    components::Velocity,
    debri::{
        components::{Collected, CollectedEvent},
        resources::DebriUniverse,
    },
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::game::{debri::components::Collider, score::resources::Score};

use super::{
    components::{Enemy, EnemySpawnEvent},
    ENEMY_SIZE,
};

pub fn enemy_movement(
    mut query: Query<(&mut Transform, &mut Enemy, &Collider, &mut Velocity), Without<Collected>>,
    mut score: ResMut<Score>,
    enemy_query: Query<&Collider, With<Enemy>>,
    universe: Res<DebriUniverse>,
    time: Res<Time>,
    mut events: EventWriter<CollectedEvent>,
) {
    let mut rng = ThreadRng::default();
    let exclude_ids = enemy_query
        .iter()
        .filter_map(|collider| collider.id.clone())
        .collect::<Vec<_>>();

    for (mut transform, mut enemy, collider, velocity) in query.iter_mut() {
        // -------------------- collision query --------------------
        let query_region = collider
            .into_region(transform.translation)
            .with_margin((universe.vision * 4000.0) as i32);
        let collisions = universe.graph.query(&query_region, &exclude_ids);

        // move towards any debri in range
        if let Some(nearest) = collisions
            .iter()
            .min_by_key(|body| (transform.translation - body.position).length_squared() as i32)
        {
            let direction = nearest.position - transform.translation;
            let mut towards = if direction.length() > 0.0 {
                direction.normalize()
            } else {
                Vec3::ZERO
            };
            // Add randomness to the movement
            towards.x += rng.gen_range(-0.2..0.2);
            towards.y += rng.gen_range(-0.2..0.2);

            transform.translation.x += towards.x * time.delta_seconds() * velocity.value.x;
            transform.translation.y += towards.y * time.delta_seconds() * velocity.value.y;

            // collision with debri
            let distance = transform.translation.distance(nearest.position);
            if distance < ENEMY_SIZE {
                events.send(CollectedEvent {
                    entity: nearest.entity,
                });

                enemy.returning = true;
                enemy.carrying = Some(1.0);
            } else {
                enemy.returning = false;
                enemy.carrying = None;
            }
        }
    }
}

pub fn despawn_enemy(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_enemy(
    mut events: EventReader<EnemySpawnEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in events.read() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(ENEMY_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_xyz(
                    event.spawn_pos.translation.x,
                    event.spawn_pos.translation.y,
                    0.0,
                ),
                ..Default::default()
            },
            Enemy {
                stash_pos: event.spawn_pos,
                returning: false,
                carrying: None,
            },
            Collider::new(ENEMY_SIZE),
            Velocity {
                value: Vec3::new(200.0, 200.0, 0.0),
                damping: 0.0,
                min_speed: 0.0,
            },
        ));
    }
}
