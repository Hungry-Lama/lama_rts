use bevy::prelude::{Plugin, App};

use crate::resources::interact_state::{InteractionStateEvent, InteractionStartsEvent};

pub mod interaction;
pub mod movement;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionStateEvent>()
        .add_event::<InteractionStartsEvent>()
        .add_system(movement::set_character_target)
        .add_system(movement::move_towards_target)
        .add_system(interaction::starts_interaction_event)
        .add_system(interaction::set_interaction)
        .add_system(interaction::set_interaction_text);
    }
}