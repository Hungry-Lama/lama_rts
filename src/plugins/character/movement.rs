use bevy::prelude::*;
use bevy_mod_picking::RayCastSource;
use bevy_mod_raycast::RayCastMethod;

use crate::{components::{movable::Movable, can_interact::CanInteract, interactible::Interactible, selectable::{Selectable, Selected}, resource_vein::ResourceVeinComponent}, InteractionStateEvent, resources::interact_state::InteractState, InteractionStartsEvent};
use crate::resources::target_type::TargetType;

pub fn move_towards_target(
    mut movable_query: Query<(
        Entity,
        &mut Transform,
        &mut Movable,
        Option<&mut CanInteract>,
    )>,
    interactible_query: Query<&Interactible>,
    mut ev_interaction_starts: EventWriter<InteractionStartsEvent>,
    time: Res<Time>,
) {
    for (movable_entity, mut transform, mut movable, can_interact) in movable_query.iter_mut() {
        match movable.target {
            Some(target_type) => {
                let (target_position, interactible) = match target_type {
                    TargetType::Position(vec) => (vec, None),
                    TargetType::Interactible(e) => {
                        if let Ok(i) = interactible_query.get(e) {
                            (i.interaction_point, Some(e))
                        } else {
                            continue
                        }
                    }
                };

                if (transform.translation - target_position).length() < 0.2 {
                    movable.target = None;
                    if let Some(interactible_entity) = interactible {
                        if let Some(_) = can_interact {
                            ev_interaction_starts.send(InteractionStartsEvent(movable_entity, interactible_entity));
                        }
                    }
                } else {
                    let translate = (target_position - transform.translation).normalize()
                        * movable.speed
                        * time.delta_seconds();
                    transform.translation += translate;
                }
            }
            None => continue,
        }
    }
}



pub fn set_character_target(
    mut raycast_source_query: Query<&mut RayCastSource<Selectable>>,
    mut interactibles_query: Query<&mut ResourceVeinComponent>,
    mut selecteds_query: Query<(Entity, &mut Movable, Option<&mut CanInteract>), With<Selected>>,
    mut ev_interaction_text: EventWriter<InteractionStateEvent>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    // Get the main window
    let window = windows.get_primary().unwrap();

    // Check if the cursor is in the main window
    if let Some(cursor_position) = window.cursor_position() {
        for mut pick_source in &mut raycast_source_query {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);

            if buttons.just_pressed(MouseButton::Right) {
                if let Some((entity, intersection)) = pick_source.intersect_top() {
                    let (new_target, interact) = if let Ok(_vein) = interactibles_query.get_mut(entity) {
                        (TargetType::Interactible(entity), true)
                    } else {
                        (TargetType::Position(intersection.position()), false)
                    };

                    for (entity, mut movable, can_interact) in selecteds_query.iter_mut() {
                        movable.target = Some(new_target);

                        match can_interact {
                            Some(mut i) if interact => {
                                i.state = InteractState::GoingToInteract;
                                ev_interaction_text.send(InteractionStateEvent(
                                    entity,
                                    InteractState::GoingToInteract,
                                ));
                            }
                            Some(mut i) => {
                                i.state = InteractState::StandBy;
                                ev_interaction_text
                                    .send(InteractionStateEvent(entity, InteractState::StandBy));
                            }
                            _ => continue,
                        }
                    }
                }
            }
        }
    }
}