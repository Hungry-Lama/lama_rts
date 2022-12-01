use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum InGameResourceType {
    Ore,
    Civilians,
    None,
}