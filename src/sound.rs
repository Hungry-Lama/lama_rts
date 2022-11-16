use bevy::{prelude::*};

pub fn start_background_music(
    asset_server: Res<AssetServer>,
    audio: ResMut<Audio>,
) {
    let music = asset_server.load("sounds/piano.ogg");
    audio.play(music);
}