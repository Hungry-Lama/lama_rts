use bevy::prelude::*;

#[derive(Component)]
pub struct Workable {
    workers: Vec<Entity>,
}