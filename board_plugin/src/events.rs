use crate::components::Coordinates;
use bevy::prelude::Event;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileTriggerEvent(pub Coordinates);
