use bevy::prelude::*;

use crate::resources::resources_enums::InGameResourceType;

#[derive(Component)]
pub struct ResourceVeinComponent {
    pub resource_type: InGameResourceType,
    pub amount: u32,
    pub workers: Vec<Entity>,
    pub timer: Timer,
}

#[derive(Component)]
pub struct CollectibleResourceUI;