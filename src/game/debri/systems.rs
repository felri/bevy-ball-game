use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;
use crate::game::components::Velocity;

use super::{
    components::{Body, Collected, Collider, Debri, SpawnDebri}, DebriUniverse, QuadBench, DEBRI_SIZE
};

pub fn build_or_update_quadtree(
    mut query: Query<(Entity, &Transform, &mut Collider, &Velocity), With<Debri>>,
    mut universe: ResMut<DebriUniverse>,
    mut bench: ResMut<QuadBench>,
) {
    let now = instant::Instant::now();
    universe.graph.clear();
    query
        .iter_mut()
        .for_each(|(entity, transform, mut collider, velocity)| {
            collider.id = Some(universe.graph.insert(
                collider.into_region(transform.translation),
                Body {
                    entity,
                    position: transform.translation,
                    velocity: velocity.value,
                },
            ));
        });
    bench.avarage_build_time = now.elapsed().as_micros();
}

// pub fn debri_movement(
//     mut commands: Commands,
//     mut debri_query: Query<(
//         Entity,
//         &mut Transform,
//         &mut Debri,
//         Option<&Collected>,
//         Option<&Collider>,
//     )>,
//     time: Res<Time>,
// ) {
//     let mut rng = rand::thread_rng();

//     for (entity, mut transform, mut debri, collected, collider) in debri_query.iter_mut() {
//         if let Some(_) = collected {
//             commands.entity(entity).despawn();
//             continue;
//         }

//         if let Some(_) = collider {
//             continue;
//         }

//         debri.time_alive += time.delta_seconds();

//         // If the velocity is too small, skip this debris
//         if debri.velocity.length() < 1.0 {
//             commands.entity(entity).insert(Collider);
//             continue;
//         }

//         // Delay the start of deceleration
//         if debri.time_alive > 1.0 {
//             // Increased from 0.5 to 1.0 seconds
//             debri.start_deceleration = true;
//         }

//         if debri.start_deceleration {
//             let deceleration_rate = rng.gen_range(200.0..900.0); // Reduced max range
//             let velocity_copy = debri.velocity.clone();
//             debri.velocity -= velocity_copy.normalize() * deceleration_rate * time.delta_seconds();
//         }

//         // Apply less frequent random direction change
//         if rng.gen_bool(0.05) {
//             // 10% chance each frame to change direction
//             let angle: f32 = rng.gen_range(-10.0f32..10.0f32).to_radians(); // Reduced angle variation
//             let new_velocity = rotate_vector(debri.velocity, angle);
//             debri.velocity = new_velocity;
//         }

//         if debri.velocity.x != 0.0 || debri.velocity.y != 0.0 {
//             transform.translation.x += debri.velocity.x * time.delta_seconds();
//             transform.translation.y += debri.velocity.y * time.delta_seconds();
//         }
//     }
// }
// // Function to rotate a vector by a given angle
fn rotate_vector(vec: Vec2, angle: f32) -> Vec2 {
    let (sin_angle, cos_angle) = angle.sin_cos();
    Vec2::new(
        cos_angle * vec.x - sin_angle * vec.y,
        sin_angle * vec.x + cos_angle * vec.y,
    )
}

pub fn despawn_debri(mut commands: Commands, projectile_query: Query<Entity, With<Debri>>) {
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_debri(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut events: EventReader<SpawnDebri>,
) {
    for event in events.read() {
        let mut rng = rand::thread_rng();
        let position = event.position.clone();
        let direction = event.direction;
        // Negate and normalize the direction for debris velocity
        let velocity = -direction.normalize() * rng.gen_range(200.0..300.0);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(DEBRI_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..default()
            },
            Debri {
                velocity: Vec2::new(velocity.x, velocity.y),
                time_alive: 0.0,
                start_deceleration: false,
            },
        ));
    }
}
