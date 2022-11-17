use bevy::prelude::*;

use crate::resources::interact_state::InteractState;

#[derive(Component)]
pub struct CanInteract {
    pub state: InteractState,
}

