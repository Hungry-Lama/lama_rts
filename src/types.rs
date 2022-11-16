use bevy::{input::{keyboard::KeyCode, Input}};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        match self {
            Directions::Up => input.just_pressed(KeyCode::Up),
            Directions::Down => input.just_pressed(KeyCode::Down),
            Directions::Left => input.just_pressed(KeyCode::Left),
            Directions::Right => input.just_pressed(KeyCode::Right),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}

impl Speed {
    pub fn value(&self) -> f32 {
        match self {
            Speed::Slow => 1.0,
            Speed::Medium => 2.0,
            Speed::Fast => 3.0,
        }
    }
}