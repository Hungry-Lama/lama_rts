use bevy::prelude::*;

use crate::resources::collectible_resource_type::CollectibleResourceType;

#[derive(Component)]
pub struct ResourceVeinComponent {
    pub resource_type: CollectibleResourceType,
    pub amount: u32,
    pub workers: Vec<Entity>,
    pub timer: Timer,
}