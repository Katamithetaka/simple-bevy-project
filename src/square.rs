use bevy::{prelude::{Bundle, Color, ResMut, Assets, Mesh, shape, Transform, Vec3}, sprite::{MaterialMesh2dBundle, ColorMaterial}};

use crate::{size::Size, position::Position2d};




#[derive(Bundle, Default)]
pub struct Square {
    pub size: Size,
    pub position: Position2d,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl Square {
    pub fn new(
        size: f32,
        position: Position2d,
        color: Color,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            size: Size { size },
            position,
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::default()
                    .with_scale(Vec3::splat(size))
                    .with_translation(Vec3 {
                        x: position.x,
                        y: position.y,
                        z: 0.0,
                    }),
                ..Default::default()
            },
        }
    }

    pub fn builder() -> SquareBuilder {
        return SquareBuilder::default();
    }
}

#[derive(Default)]
pub struct SquareBuilder {
    size: f32,
    position: Position2d,
    color: Color,
}

impl SquareBuilder {
    pub fn with_size(self, size: f32) -> Self {
        return Self { size, ..self };
    }

    pub fn with_position(self, position: Position2d) -> Self {
        return Self { position, ..self };
    }

    pub fn with_color(self, color: Color) -> Self {
        return Self { color, ..self };
    }

    pub fn build(
        self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Square {
        return Square::new(self.size, self.position, self.color, meshes, materials);
    }
}