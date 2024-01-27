use crate::{
    game::{
        components::Velocity,
        debri::{components::{Collected, CollectedEvent}, resources::DebriUniverse},
    },
    quadtree::slot_map::SlotId,
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
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Collector, &Collider, &mut Velocity), Without<Collected>>,
    mut score: ResMut<Score>,
    collector_query: Query<&Collider, With<Collector>>,
    universe: Res<DebriUniverse>,
    time: Res<Time>,
    mut events: EventWriter<CollectedEvent>,
) {
    let mut rng = ThreadRng::default();

    for (entity, mut transform, collector, collider, velocity) in query.iter_mut() {
        // if collected debri and is returning
        if collector.returning {
            let distance = transform
                .translation
                .distance(collector.stash_pos.translation);
            // if reached stash
            if distance < COLLECTOR_SIZE {
                commands.entity(entity).insert((
                    Collector {
                        stash_pos: collector.stash_pos,
                        returning: false,
                        carrying: None,
                    },
                    velocity.clone(),
                ));

                score.value += 1;
            } else {
                let mut towards =
                    (collector.stash_pos.translation - transform.translation).normalize();

                // Add randomness to the movement
                towards.x += rng.gen_range(-0.2..0.2);
                towards.y += rng.gen_range(-0.2..0.2);

                transform.translation.x += towards.x * time.delta_seconds() * velocity.value.x;
                transform.translation.y += towards.y * time.delta_seconds() * velocity.value.y;
            }

            continue;
        }

        let exclude_ids = collector_query
            .iter()
            .filter_map(|collider| collider.id.clone())
            .collect::<Vec<_>>();

        // -------------------- collision query --------------------
        let query_region = collider
            .into_region(transform.translation)
            .with_margin((universe.vision * 4000.0) as i32);
        let exclude = match &collider.id {
            Some(id) => {
                // add collector id to exclude list
                let mut e = vec![id.clone()];
                e.extend(exclude_ids);
                e
            }
            None => vec![],
        };

        let collisions = universe.graph.query(&query_region, &exclude);

        // move towards any debri in range
        if let Some(nearest) = collisions
            .iter()
            .min_by_key(|body| (transform.translation - body.position).length_squared() as i32)
        {
            let mut towards = (nearest.position - transform.translation).normalize();

            // Add randomness to the movement
            towards.x += rng.gen_range(-0.2..0.2);
            towards.y += rng.gen_range(-0.2..0.2);

            transform.translation.x += towards.x * time.delta_seconds() * velocity.value.x;
            transform.translation.y += towards.y * time.delta_seconds() * velocity.value.y;
        }

        // collision with debri
        for body in collisions.iter() {
            let distance = transform.translation.distance(body.position);
            if distance < COLLECTOR_SIZE {
                events.send(CollectedEvent {
                    entity: body.entity,
                });

                commands.entity(entity).insert((
                    Collector {
                        stash_pos: collector.stash_pos,
                        returning: true,
                        carrying: Some(1.0),
                    },
                    velocity.clone(),
                ));
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
