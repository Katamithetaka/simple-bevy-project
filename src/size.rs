use bevy::prelude::Component;

#[derive(Component, Default, Clone, Copy)]
pub struct Size {
    pub size: f32
}

impl Size {
    pub fn new(size: f32) -> Size {
        Self { size }
    }
}