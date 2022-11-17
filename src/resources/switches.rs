use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum SwitchType {
    TechTestResearched,
}