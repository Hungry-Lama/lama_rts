use bevy::utils::HashMap;
use crate::resources::techs_enums::Techs;

pub enum SetResourceError {
    MaxCapacityPartiallyExceeded,
    MaxCapacityExceeded,
    UnderZero,
}

#[derive(Default)]
pub struct PlayerData {
    ore: u32,
    pub max_ore: u32,
    pub techs: HashMap<Techs, bool>,
}

impl PlayerData {
    pub fn ore(&self) -> u32 {
        self.ore
    }

    pub fn add_ore(&mut self, ore: u32) -> Result<(), SetResourceError> {
        if self.ore >= self.max_ore {
            return Err(SetResourceError::MaxCapacityExceeded)
        } else if self.ore + ore > self.max_ore {
            self.ore += ore;
            return Err(SetResourceError::MaxCapacityPartiallyExceeded);
        } else {
            self.ore += ore;
            return Ok(());
        }
    }

    pub fn sub_ore(&mut self, ore: u32) -> Result<(), SetResourceError> {
        if self.ore - ore < 0 {
            return Err(SetResourceError::UnderZero);
        } else {
            self.ore -= ore;
            return Ok(());
        }
    }

    pub fn set_ore(&mut self, ore: u32) -> Result<(), SetResourceError> {
        self.ore = ore;
        return Ok(());
    }
}