use bevy::prelude::*;
use bevy_text_mesh::TextMesh;

use crate::{resources::{interact_state::{InteractState, InteractionStartsEvent, InteractionStateEvent}}, components::{interactible::Interactible, can_interact::CanInteract, resource_vein::ResourceVeinComponent}};

pub fn starts_interaction_event(
    mut ev_interaction_starts: EventReader<InteractionStartsEvent>,
    mut ev_interaction_text: EventWriter<InteractionStateEvent>,
    mut q_interactible: Query<&mut ResourceVeinComponent, With<Interactible>>,
    q_interactor: Query<&CanInteract>,
) {
    for ev in ev_interaction_starts.iter() {
        if let Ok(_) = q_interactor.get(ev.0) {
            if let Ok(mut resource_vein) = q_interactible.get_mut (ev.1) {
                ev_interaction_text
                .send(InteractionStateEvent(ev.0, InteractState::Interacting));

                resource_vein.workers.push(ev.0);
            }
        }
    }
}

pub fn set_interaction (
    mut ev_interaction_state: EventReader<InteractionStateEvent>,
    mut q_interactor: Query<&mut CanInteract>,
) {
    for ev in ev_interaction_state.iter() {
        if let Ok(mut interactor) = q_interactor.get_mut(ev.0) {
            interactor.state = ev.1;
        }
    }
}

pub fn set_interaction_text(
    mut ev_interaction_text: EventReader<InteractionStateEvent>,
    q_parent: Query<&Children>,
    mut q_child: Query<&mut TextMesh>,
) {
    for ev in ev_interaction_text.iter() {
        if let Ok(component) = q_parent.get(ev.0) {
            for &child in component {
                if let Ok(mut text) = q_child.get_mut(child) {
                    match ev.1 {
                        InteractState::StandBy => text.text = String::from("Not interacting"),
                        InteractState::GoingToInteract => {
                            text.text = String::from("Going to interact")
                        }
                        InteractState::Interacting => text.text = String::from("Interacting"),
                    }
                }
            }
        }
    }
}