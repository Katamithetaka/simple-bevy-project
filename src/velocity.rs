use bevy::prelude::{Vec2, Component};

#[derive(Component, Default, Copy, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default, Copy, Clone)]
pub struct MaxVelocity(pub Vec2);

#[derive(Component, Default, Copy, Clone)]
pub struct Acceleration(pub Vec2);

