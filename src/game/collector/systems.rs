use crate::{
    game::{
        components::Velocity,
        debri::{self, resources::DebriUniverse},
    },
    quadtree::slot_map::SlotId,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::game::{
    debri::components::{Collected, Collider, Debri},
    score::resources::Score,
};

use super::{
    components::{Collector, CollectorSpawnEvent},
    COLLECTOR_SIZE,
};

// pub fn collector_movement(
//     mut query: Query<(&mut Transform, &Collector)>,
//     time: Res<Time>,
//     treeaccess: Res<NNTree>,
// ) {
//     let mut rng = ThreadRng::default(); // Create a new random number generator

//     for (mut transform, collector) in query.iter_mut() {
//         let collector_pos = Vec2::new(transform.translation.x, transform.translation.y);
//         let target_pos = if collector.returning {
//             Vec2::new(
//                 collector.stash_pos.translation.x,
//                 collector.stash_pos.translation.y,
//             )
//         } else if let Some(nearest) = treeaccess.nearest_neighbour(collector_pos) {
//             nearest.0
//         } else {
//             collector_pos
//         };

//         let mut towards = (target_pos - collector_pos).normalize();

//         // Add randomness to the movement
//         towards.x += rng.gen_range(-0.2..0.2);
//         towards.y += rng.gen_range(-0.2..0.2);

//         transform.translation.x += towards.x * time.delta_seconds() * collector.velocity;
//         transform.translation.y += towards.y * time.delta_seconds() * collector.velocity;
//     }
// }

pub fn collector_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Collector, &Collider)>,
    mut score: ResMut<Score>,
    collector_query: Query<&Collider, With<Collector>>,
    universe: Res<DebriUniverse>,
    time: Res<Time>,
) {
    let mut rng = ThreadRng::default();

    for (entity, mut transform, collector, collider) in query.iter_mut() {
        // if collected debri and is returning
        if collector.returning {
            let distance = transform
                .translation
                .distance(collector.stash_pos.translation);
            // if reached stash
            if distance < COLLECTOR_SIZE {
                commands.entity(entity).insert(Collector {
                    velocity: collector.velocity,
                    stash_pos: collector.stash_pos,
                    returning: false,
                    carrying: None,
                });

                score.value += 1;
            } else {
                let mut towards =
                    (collector.stash_pos.translation - transform.translation).normalize();

                // Add randomness to the movement
                towards.x += rng.gen_range(-0.2..0.2);
                towards.y += rng.gen_range(-0.2..0.2);

                transform.translation.x += towards.x * time.delta_seconds() * collector.velocity;
                transform.translation.y += towards.y * time.delta_seconds() * collector.velocity;
            }

            continue;
        }

        // -------------------- collision query --------------------
        let query_region = collider
            .into_region(transform.translation)
            .with_margin((universe.vision * 700.0) as i32);
        let exclude = match &collider.id {
            Some(id) => {
                // add collector id to exclude list
                let mut e = vec![id.clone()];
                e.extend(
                    collector_query
                        .iter()
                        .map(|collider| collider.id.clone().unwrap())
                        .collect::<Vec<_>>(),
                );
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

            transform.translation.x += towards.x * time.delta_seconds() * collector.velocity;
            transform.translation.y += towards.y * time.delta_seconds() * collector.velocity;
        }

        // collision with debri
        for body in collisions.iter() {
            let distance = transform.translation.distance(body.position);
            if distance < COLLECTOR_SIZE {
                commands.entity(body.entity).despawn();

                commands.entity(entity).insert(Collector {
                    velocity: collector.velocity,
                    stash_pos: collector.stash_pos,
                    returning: true,
                    carrying: Some(1.0),
                });
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
                velocity: 100.0,
                stash_pos: event.spawn_pos,
                returning: false,
                carrying: None,
            },
            Collider::new(COLLECTOR_SIZE),
            Velocity {
                value: Vec3::new(0.0, 0.0, 0.0),
                damping: 0.0,
                min_speed: 0.0,
            },
        ));
    }
}

// pub fn check_colision_collector(
//     mut commands: Commands,
//     query: Query<(Entity, &Transform, &Collector)>,
//     query_debri: Query<(Entity, &Transform, &Collider)>,
//     mut score: ResMut<Score>,
// ) {
//     for (entity, transform, collector) in query.iter() {
//         if collector.returning {
//             let distance = transform
//                 .translation
//                 .distance(collector.stash_pos.translation);
//             if distance < COLLECTOR_SIZE {
//                 commands.entity(entity).insert(Collector {
//                     velocity: collector.velocity,
//                     stash_pos: collector.stash_pos,
//                     returning: false,
//                     carrying: None,
//                 });

//                 score.value += 1;
//             }
//         } else {
//             // -------------------- collision query --------------------
//             let query_region = collider
//                 .into_region(transform.translation);
//             let exclude = match &collider.id {
//                 Some(id) => vec![id.clone()],
//                 None => vec![],
//             };

//         } else {
//             for (entity_debri, transform_debri) in query_debri.iter() {
//                 let distance = transform.translation.distance(transform_debri.translation);
//                 if distance < COLLECTOR_SIZE {
//                     commands.entity(entity_debri).insert(Collected);

//                     commands.entity(entity).insert(Collector {
//                         velocity: collector.velocity,
//                         stash_pos: collector.stash_pos,
//                         returning: true,
//                         carrying: Some(1.0),
//                     });
//                 }
//             }
//         }
//     }
// }
