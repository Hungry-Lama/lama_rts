use bevy::{utils::HashMap};
use serde::{Serialize, Deserialize};

use super::{resources_enums::InGameResourceType, switches::SwitchType};

#[derive(Serialize, Deserialize, Clone)]
pub enum DialogChoiceEvent {
    Random((u32, u32), (u32, u32)),
    Goto(u32),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DialogCondition {
    ResourceCheck(InGameResourceType, u32),
    SwitchCheck(SwitchType, bool),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DialogConsequences {
    AddResource(InGameResourceType, u32),
    SubResource(InGameResourceType, u32),
    SetResource(InGameResourceType, u32),
    SetSwitch(SwitchType, bool),
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct DialogData {
    pub text: String,
    pub choices: Vec<DialogChoice>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct DialogChoice {
    pub text: String,
    pub conditions: Option<Vec<DialogCondition>>,
    pub consequences: Option<Vec<DialogConsequences>>,
    pub id_next: Option<DialogChoiceEvent>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CurrentDialog {
    pub dialog: Option<DialogData>
}

#[derive(Default, Serialize, Deserialize)]
pub struct DialogDatas { 
    pub dialogs: HashMap<u32, DialogData>
}