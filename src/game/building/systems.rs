use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use super::{
    components::{Building, EvenetSpawnBuilding},
    BUILDING_SIZE,
};

pub fn despawn_building(mut commands: Commands, query: Query<Entity, With<Building>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_building(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut events: EventReader<EvenetSpawnBuilding>,
) {
    for event in events.read() {
        let position = event.position.clone();
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(
                        shape::Cube {
                            size: BUILDING_SIZE,
                        }
                        .into(),
                    )
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_xyz(position.x, position.y, position.z),
                ..Default::default()
            },
            Building,
        ));
    }
}
