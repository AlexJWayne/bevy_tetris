use bevy::{prelude::*, render::camera::*};

#[derive(Component)]
pub struct GameCamera;

pub fn create_camera(mut commands: Commands) {
    let projection = OrthographicProjection {
        far: 1000.,
        near: -1000.,
        scaling_mode: ScalingMode::FixedVertical(24.),
        ..default()
    };

    let transform = Transform::from_xyz(5., 9., 0.);

    commands.spawn((
        Camera2dBundle {
            projection,
            transform,
            ..default()
        },
        GameCamera,
    ));
}
