use bevy::prelude::{Vec2, Vec3, Component};

#[derive(Component, Default, Clone, Copy)]
pub struct Position2d {
    pub x: f32,
    pub y: f32,
}

impl Position2d {
    pub fn from_vec3(vec3: Vec3) -> Self {
        Self {
            x: vec3.x / vec3.z,
            y: vec3.y / vec3.z
        }
    }

    pub fn lossy_from_vec3(vec3: Vec3) -> Self {
        Self {
            x: vec3.x,
            y: vec3.y
        }
    }
}

impl Into<Vec3> for Position2d {
    fn into(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: 1.0,
        }
    }
}

impl Into<Vec2> for Position2d {
    fn into(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y
        }
    }
}

impl From<Vec2> for Position2d {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}


