use bevy::{utils::HashMap, prelude::ResMut};

use crate::resources::{self, techs_enums::Techs};

pub fn setup(
    mut player_data: ResMut<resources::player::data::PlayerData>,
    mut camera_data: ResMut<resources::camera::CameraData>,
) {
    camera_data.speed = 20.0;
    player_data.set_ore(15);
    player_data.max_ore = 25;
    player_data.techs = HashMap::new();
    player_data.techs.insert(Techs::BetterOreMining, true);
    player_data.techs.insert(Techs::DialogPossibility, false);
}