use bevy::prelude::Entity;

#[derive(Clone, Copy)]
pub enum InteractState {
    StandBy,
    GoingToInteract,
    Interacting,
}

pub struct InteractionStateEvent(pub Entity, pub InteractState);
pub struct InteractionStartsEvent(pub Entity, pub Entity);