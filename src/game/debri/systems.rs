use crate::game::{collector::components::Collector, components::Velocity};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;
use std::f32::consts::PI;

use super::{
    components::{Body, Collected, CollectedEvent, Collider, Debri, SpawnDebri},
    DebriUniverse, DEBRI_SIZE,
};

pub fn build_or_update_quadtree(
    mut query: Query<(Entity, &Transform, &mut Collider, &Velocity), Without<Collector>>,
    mut universe: ResMut<DebriUniverse>,
) {
    universe.graph.clear();
    query
        .iter_mut()
        .for_each(|(entity, transform, mut collider, velocity)| {
            let id = Some(universe.graph.insert(
                collider.into_region(transform.translation),
                Body {
                    entity,
                    position: transform.translation,
                    velocity: velocity.value,
                },
            ));
            collider.id = id;
        });
}

pub fn update_debri(
    mut query: Query<(Entity, &Transform, &mut Collider, &mut Velocity), Without<Collector>>,
    universe: Res<DebriUniverse>,
    time: Res<Time>,
) {
    query
        .iter_mut()
        .for_each(|(_entity, transform, mut collider, mut velocity)| {
            // if collected
            let x = transform.translation.x as i32;
            let y = transform.translation.y as i32;
            let win = universe.graph.size();

            // -------------------- collision query --------------------
            let query_region = collider
                .into_region(transform.translation)
                .with_margin((universe.vision * 10.0) as i32);
            let exclude = match &collider.id {
                Some(id) => vec![id.clone()],
                None => vec![],
            };

            let collisions = universe.graph.query(&query_region, &exclude);
            collider.nearby = collisions.len();

            let (mass_center, aligment, separtion) = collisions.iter().fold(
                (Vec3::ZERO, Vec3::ZERO, Vec3::ZERO),
                |(mcen, alg, sep), body| {
                    (
                        mcen + body.position.normalize(),
                        alg + body.velocity.normalize(),
                        sep + (transform.translation - body.position).normalize(),
                    )
                },
            );

            let mut direction = velocity.value.normalize();

            // -------------------- Cohesion --------------------
            if mass_center.length() > 0.0 {
                direction += (mass_center.normalize() - transform.translation.normalize())
                    .normalize()
                    * universe.cohesion;
            }

            // -------------------- Alignment --------------------
            if aligment.length() > 0.0 {
                direction += aligment.normalize() * universe.alignment;
            }

            // -------------------- Separation --------------------
            if separtion.length() > 0.0 {
                direction += separtion.normalize() * universe.speration;
            }

            let mut new_velocity = direction.normalize() * velocity.value.length();

            // -------------------- World Border --------------------
            let margin: i32 = 20;
            if (x < win.min.x + margin && velocity.value.x < 0.0)
                || (x > win.max.x - margin && velocity.value.x > 0.0)
            {
                new_velocity.x *= -1.0;
            }
            if (y < win.min.y + margin && velocity.value.y < 0.0)
                || (y > win.max.y - margin && velocity.value.y > 0.0)
            {
                new_velocity.y *= -1.0;
            }

            // -------------------- Random --------------------
            if rand::thread_rng().gen_range(0..100) < 1 {
                new_velocity.x += rand::thread_rng().gen_range(-10.0..10.0);
                new_velocity.y += rand::thread_rng().gen_range(-10.0..10.0);
            }

            // -------------------- Damping --------------------
            let delta_time = universe.speed * time.delta_seconds();
            let mut damping = velocity.damping * delta_time;
            if new_velocity.length() < velocity.min_speed {
                damping *= 0.1;
            }
            new_velocity -= new_velocity.normalize() * damping;

            // finally set the new velocity
            velocity.value = new_velocity;
        });
}

pub fn move_system(
    mut query: Query<(&mut Transform, &Velocity), Without<Collector>>,
    universe: Res<DebriUniverse>,
    time: Res<Time>,
) {
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        let direction = velocity.value.normalize();
        let rotation = Quat::from_rotation_z(-direction.x.atan2(direction.y) + PI / 2.0);
        transform.rotation = rotation;
        transform.translation += velocity.value * time.delta_seconds() * universe.speed;
    });
}

pub fn render_quadtree(_commands: Commands, universe: ResMut<DebriUniverse>, mut gizmos: Gizmos) {
    let regions = universe.graph.get_regions();

    regions.iter().for_each(|region| {
        let (min_x, min_y, max_x, max_y) = region.into_f32();

        let bottom_left = Vec3::new(min_x, min_y, 0.0);
        let bottom_right = Vec3::new(max_x, min_y, 0.0);
        let top_right = Vec3::new(max_x, max_y, 0.0);
        let top_left = Vec3::new(min_x, max_y, 0.0);

        gizmos.line(bottom_left, bottom_right, Color::WHITE);
        gizmos.line(bottom_right, top_right, Color::WHITE);
        gizmos.line(top_right, top_left, Color::WHITE);
        gizmos.line(top_left, bottom_left, Color::WHITE);
    })
}

pub fn despawn_debri(mut commands: Commands, projectile_query: Query<Entity, With<Debri>>) {
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_debri_collected_event(
    mut commands: Commands,
    mut events: EventReader<CollectedEvent>,
    mut query: Query<(Entity, &mut Transform, &mut Collider, &mut Velocity), Without<Collector>>,
    mut universe: ResMut<DebriUniverse>,
) {
    for event in events.read() {
        let entity = event.entity;
        let collider = query.get_mut(entity);
        if let Ok((_, _, collider, _)) = collider {
            let slot_id = collider.id.as_ref().unwrap();
            universe.graph.remove(&slot_id);
            commands.entity(entity).despawn();
        }
    }
}

pub fn count_debri(query: Query<&Debri, Without<Collected>>, mut universe: ResMut<DebriUniverse>) {
    universe.debri_count = query.iter().count() as u32;
}

pub fn spawn_debri(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut events: EventReader<SpawnDebri>,
) {
    for event in events.read() {
        let position = event.position.clone();
        let direction = event.direction;
        let initial_speed = 200.0;
        // Negate and normalize the direction for debris velocity
        let velocity = Vec3::new(
            -direction.x * initial_speed * 1.2,
            -direction.y * initial_speed * 1.2,
            0.0,
        );

        commands
            .spawn(MaterialMesh2dBundle {
                // texture: assets.load("boid.png"),
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(
                        DEBRI_SIZE, DEBRI_SIZE,
                    ))))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::rgb(2., 2., 0.))),
                // texture: assets.load("/files/assets/boid.png"),
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..Default::default()
            })
            .insert(Debri)
            .insert(Velocity {
                value: velocity,
                damping: 50.0,
                min_speed: 50.0,
            })
            .insert(Collider::new(DEBRI_SIZE));
    }
}
