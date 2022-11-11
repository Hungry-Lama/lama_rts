use bevy::prelude::*;

use crate::resources::target_type::TargetType;

#[derive(Component)]
pub struct Movable {
    pub speed: f32,
    pub target: Option<TargetType>,
}

