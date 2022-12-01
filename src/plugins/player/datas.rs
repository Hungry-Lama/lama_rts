use bevy::{prelude::{Query, Res, With}, text::Text};

use crate::{resources, components::resource_vein::CollectibleResourceUI};

pub fn update_ore_ui(player_data: Res<resources::player::data::PlayerData>, mut texts: Query<&mut Text, With<CollectibleResourceUI>>) {
    for mut text in texts.iter_mut() {
        text.sections[0].value = format!("Ore: {}/{}", player_data.ore(), player_data.max_ore);
    }
}