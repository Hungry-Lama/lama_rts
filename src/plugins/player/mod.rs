use bevy::prelude::{Plugin, App};

use crate::resources::{player::{camera::CameraData, data::PlayerData}};

pub mod player_data;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerData>()
        .init_resource::<CameraData>();
    }
}