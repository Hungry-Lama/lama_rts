use bevy::prelude::Component;

#[derive(Component)]
pub struct DialogChoiceButton {
    pub id: usize,
    pub enabled: bool,
}