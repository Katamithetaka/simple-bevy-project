pub mod position;
pub mod square;
pub mod size;
pub mod velocity;
use position::Position2d;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use size::Size;
use square::Square;
use velocity::{Velocity, MaxVelocity, Acceleration};



const SQUARE_SIZE: f32 = 50.;
const BORDER_SIZE: f32 = 10.;
const BASE_GAME_HEIGHT: f32 = 400.;
const BASE_GAME_WIDTH: f32 = 200.;
const GAME_HEIGHT: f32 = BASE_GAME_HEIGHT + SQUARE_SIZE + 2.;
const WINDOW_HEIGHT: f32 = GAME_HEIGHT + BORDER_SIZE * 2.;
const GAME_WIDTH: f32 = BASE_GAME_WIDTH + BORDER_SIZE;
const TOP:f32 = BASE_GAME_HEIGHT / 2.;
const BOTTOM:f32 = -(BASE_GAME_HEIGHT / 2.);
const LEFT: f32 = -(GAME_WIDTH / 2. - BORDER_SIZE);
const RIGHT: f32  = GAME_WIDTH / 2. - BORDER_SIZE;



fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Acceleration, &mut Velocity, &MaxVelocity, &mut Position2d)>) {
    for (mut acceleration, mut velocity, max, mut position) in &mut sprite_position {
        
        position.y += velocity.0.y * time.delta_seconds() / 2.;
        velocity.0.y += acceleration.0.y * time.delta_seconds() / 2.;

        if velocity.0.y.abs() > max.0.y {
            velocity.0.y = max.0.y * velocity.0.y.signum();
            acceleration.0.y *= -1.;
        }

        if position.y > TOP {
            velocity.0.y *= -1.;
            acceleration.0.y *= -1.;

        } else if position.y < BOTTOM {
            velocity.0.y *= -1.;
            acceleration.0.y *= -1.;

        }

        position.y = position.y.max(BOTTOM).min(TOP);
    }
}

fn scale_square(time: Res<Time>, mut query: Query<(&mut Size, &Velocity, &MaxVelocity)>) {
    for (mut size, velocity, max) in &mut query {
        size.size = (time.elapsed_seconds() * (velocity.0.y / max.0.y * 5.0)).sin().abs() * 50.;
    }
}
fn update_position(mut query: Query<(&Position2d, &mut Transform)>) {
    for (position, mut transform) in &mut query {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}


fn update_size(mut query: Query<(&Size, &mut Transform)>) {
    for (Size { size }, mut transform) in &mut query {
        transform.scale.x = *size; 
        transform.scale.y = *size; 
        transform.scale.z = *size;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Simple test game".to_string(),
                    resolution: (GAME_WIDTH, WINDOW_HEIGHT).into(),
                    resize_constraints: WindowResizeConstraints {
                        min_height: WINDOW_HEIGHT,
                        min_width: GAME_WIDTH,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                close_when_requested: true,
                ..Default::default()
            })
        )
        .add_systems(Startup, init_border)
        .add_systems(Startup, setup_square)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, scale_square)
        .add_systems(Update, update_position)
        .add_systems(Update, update_size)
        .run();
}

fn init_border(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let base = MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        material: materials.add(ColorMaterial::from(Color::rgba(0.7, 0.0, 0.0, 1.0))),
        transform: Transform::default()
            .with_scale(Vec3::new(BORDER_SIZE, BORDER_SIZE, 1.0))
            .with_translation(Vec3 {
                x: 0.,
                y: 0.,
                z: 0.0,
            }),
        ..Default::default()
    };

    let mut left_border = base.clone();
    left_border.transform = left_border.transform.with_translation(Vec3 {
        x: -BASE_GAME_WIDTH / 2.,
        ..Default::default()
    }).with_scale(Vec3 {
        x: BORDER_SIZE,
        y: WINDOW_HEIGHT,
        ..Default::default()
    });
    left_border.mesh = meshes.add(Mesh::from(shape::Quad::default())).into();

    let mut right_border = base.clone();
    right_border.transform = right_border.transform.with_translation(Vec3 {
        x: BASE_GAME_WIDTH / 2.,
        ..Default::default()
    }).with_scale(Vec3 {
        x: BORDER_SIZE,
        y: WINDOW_HEIGHT,
        ..Default::default()
    });
    right_border.mesh = meshes.add(Mesh::from(shape::Quad::default())).into();

    let mut top_border = base.clone();
    top_border.transform = top_border.transform.with_translation(Vec3 {
        y: -(WINDOW_HEIGHT / 2. - BORDER_SIZE / 2.),
        ..Default::default()
    }).with_scale(Vec3 {
        x: GAME_WIDTH,
        y: BORDER_SIZE,
        ..Default::default()
    });

    let mut bottom_border = base.clone();
    bottom_border.transform = bottom_border.transform.with_translation(Vec3 {
        y: (WINDOW_HEIGHT / 2. - BORDER_SIZE / 2.),
        ..Default::default()
    }).with_scale(Vec3 {
        x: GAME_WIDTH,
        y: BORDER_SIZE,
        ..Default::default()
    });
    bottom_border.mesh = meshes.add(Mesh::from(shape::Quad::default())).into();


    commands.spawn(left_border);
    commands.spawn(right_border);
    commands.spawn(top_border);
    commands.spawn(bottom_border);


}

fn setup_square(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let square = Square::builder()
        .with_color(Color::Rgba {
            red: 1.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        })
        .with_size(SQUARE_SIZE)
        .with_position(Position2d::default())
        .build(&mut meshes, &mut materials);

    commands.spawn((square,
        Velocity(Vec2::splat(5.)),
        MaxVelocity(Vec2::splat(1500.0)),
        Acceleration(Vec2::splat(50.0)),
    ));
}
