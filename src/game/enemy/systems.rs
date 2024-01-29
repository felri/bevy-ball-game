use crate::game::{
    collector::components::Collector,
    components::Velocity,
    debri::{
        components::{Collected, CollectedEvent, Debri},
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
    mut query: Query<(&mut Transform, &Collider, &mut Velocity), With<Enemy>>,
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

    for (mut transform, collider, velocity) in query.iter_mut() {
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
            let distance = direction.length();
            let desired_distance = ENEMY_SIZE * 10.0; // replace X with the desired multiplier

            // Only move towards the debris if we are further than the desired distance
            if distance > desired_distance {
                let mut towards = if distance > 0.0 {
                    direction.normalize()
                } else {
                    Vec3::ZERO
                };
                // Add randomness to the movement
                towards.x += rng.gen_range(-0.2..0.2);
                towards.y += rng.gen_range(-0.2..0.2);

                // Interpolate the speed based on the current distance to the target
                let speed_factor = (distance - desired_distance) / (desired_distance);
                let interpolated_speed = velocity.value * speed_factor.clamp(0.0, 1.0);

                transform.translation.x += towards.x * time.delta_seconds() * interpolated_speed.x;
                transform.translation.y += towards.y * time.delta_seconds() * interpolated_speed.y;
            } else {
                // If we are too close to the debris, move in the opposite direction
                let mut away = if distance > 0.0 {
                    -direction.normalize() // Note the negative sign to move in the opposite direction
                } else {
                    Vec3::ZERO
                };

                // Add randomness to the movement
                away.x += rng.gen_range(-0.2..0.2);
                away.y += rng.gen_range(-0.2..0.2);

                let speed_factor = (desired_distance - distance) / (desired_distance);
                let interpolated_speed = velocity.value * speed_factor.clamp(0.0, 1.0);

                transform.translation.x += away.x * time.delta_seconds() * interpolated_speed.x;
                transform.translation.y += away.y * time.delta_seconds() * interpolated_speed.y;
            }

            // collision with debri
            let distance = transform.translation.distance(nearest.position);
            if distance < ENEMY_SIZE * 10.0 {
                println!("Enemy hit debri");
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
                material: materials.add(ColorMaterial::from(Color::YELLOW)),
                transform: Transform::from_xyz(
                    event.position.translation.x,
                    event.position.translation.y,
                    1.0,
                ),
                ..Default::default()
            },
            Enemy,
            Collider::new(ENEMY_SIZE),
            Velocity {
                value: Vec3::new(200.0, 200.0, 0.0),
                damping: 0.0,
                min_speed: 0.0,
            },
        ));
    }
}
