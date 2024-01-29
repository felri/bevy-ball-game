use bevy::prelude::*;

use crate::game::building::components::EventSpawnBuilding;
use crate::game::collector::components::CollectorSpawnEvent;
use crate::game::enemy::components::EnemySpawnEvent;
use crate::game::ui::spawn_toolbar::components::*;
use crate::game::ui::spawn_toolbar::styles::HOVERED_BUTTON;
use crate::game::ui::spawn_toolbar::styles::NORMAL_BUTTON;
use crate::game::ui::spawn_toolbar::styles::PRESSED_BUTTON;

pub fn interact_with_button(
    mut events_spawn_collector: EventWriter<CollectorSpawnEvent>,
    mut events_spawn_building: EventWriter<EventSpawnBuilding>,
    mut events_spawn_enemy: EventWriter<EnemySpawnEvent>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &DefaultButton),
        (Changed<Interaction>, With<DefaultButton>),
    >,
) {
    for (interaction, mut color, default_button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();

                // check the enum type of default button
                match default_button {
                    DefaultButton::Collector => {
                        events_spawn_collector.send(CollectorSpawnEvent {
                            spawn_pos: Transform::from_xyz(100.0, 100.0, 0.0),
                        });
                    }
                    DefaultButton::Shooter => {
                        // events_spawn_shooter.send(SpawnShooter);
                        println!("Shooter");
                    }
                    DefaultButton::Building(building_type) => match building_type {
                        BuldingType::Collector => {
                            events_spawn_building.send(EventSpawnBuilding {
                                position: Transform::from_xyz(100.0, 100.0, 1.0),
                            });
                        }
                        BuldingType::Shooter => {
                            events_spawn_enemy.send(EnemySpawnEvent {
                                position: Transform::from_xyz(100.0, 100.0, 1.0),
                            });
                        }
                    },
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
