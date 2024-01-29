use crate::game::{
    components::Velocity,
    debri::{components::CollectedEvent, resources::DebriUniverse},
    enemy::components::Enemy,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::game::{debri::components::Collider, score::resources::Score};

use super::{
    components::{Collector, CollectorSpawnEvent},
    COLLECTOR_SIZE,
};

pub fn collector_movement(
    mut query: Query<(&mut Transform, &mut Collector, &Collider, &mut Velocity)>,
    mut score: ResMut<Score>,
    npc_query: Query<&Collider, (With<Enemy>, With<Collector>)>,
    universe: Res<DebriUniverse>,
    time: Res<Time>,
    mut events: EventWriter<CollectedEvent>,
) {
    let mut rng = ThreadRng::default();
    let exclude_ids = npc_query
        .iter()
        .filter_map(|collider| collider.id.clone())
        .collect::<Vec<_>>();

    for (mut transform, mut collector, collider, velocity) in query.iter_mut() {
        if collector.returning {
            let distance = transform
                .translation
                .distance(collector.stash_pos.translation);
            // if reached stash
            if distance < COLLECTOR_SIZE {
                collector.returning = false;
                collector.carrying = None;

                score.value += 1;
            } else {
                let direction = collector.stash_pos.translation - transform.translation;
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
            }
        } else {
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
                if distance < COLLECTOR_SIZE {
                    events.send(CollectedEvent {
                        entity: nearest.entity,
                    });

                    collector.returning = true;
                    collector.carrying = Some(1.0);
                } else {
                    collector.returning = false;
                    collector.carrying = None;
                }
            }
        }
    }
}

pub fn despawn_collector(mut commands: Commands, query: Query<Entity, With<Collector>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_collector(
    mut events: EventReader<CollectorSpawnEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in events.read() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(COLLECTOR_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_xyz(
                    event.spawn_pos.translation.x,
                    event.spawn_pos.translation.y,
                    0.0,
                ),
                ..Default::default()
            },
            Collector {
                stash_pos: event.spawn_pos,
                returning: false,
                carrying: None,
            },
            Collider::new(COLLECTOR_SIZE),
            Velocity {
                value: Vec3::new(200.0, 200.0, 0.0),
                damping: 0.0,
                min_speed: 0.0,
            },
        ));
    }
}
