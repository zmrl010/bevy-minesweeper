use bevy::prelude::*;

#[cfg_attr(feature = "debug", derive(Reflect))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Bomb;
