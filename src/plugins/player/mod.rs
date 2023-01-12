use bevy::prelude::{Plugin, App};

use crate::resources::player::data::PlayerData;

pub mod setup;
pub mod datas;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerData>()
        .add_startup_system(setup::setup)
        .add_system(datas::update_ore_ui);
    }
}