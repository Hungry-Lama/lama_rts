use bevy::{utils::HashMap};
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct CameraData {
    pub speed: f32,
}

#[derive(Default)]
pub struct PlayerData {
    pub ore: u32,
    pub max_ore: u32,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct DialogData {
    pub text: String,
    pub id_next: Option<u32>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CurrentDialog {
    pub dialog: Option<DialogData>
}

#[derive(Default, Serialize, Deserialize)]
pub struct DialogDatas { 
    pub dialogs: HashMap<u32, DialogData>
}