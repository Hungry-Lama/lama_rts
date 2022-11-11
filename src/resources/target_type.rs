use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum TargetType {
    Interactible(Entity),
    Position(Vec3),
}