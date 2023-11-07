use bevy::{prelude::*, render::camera::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (create_camera, create_stage))
        .run();
}

#[derive(Component)]
struct GameCamera;

fn create_camera(mut commands: Commands) {
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

fn create_stage(mut commands: Commands) {
    for y in [-1, 20] {
        for x in -1..11 {
            commands.spawn(build_brick_sprite(x, y));
        }
    }

    for y in 0..20 {
        for x in [-1, 10] {
            commands.spawn(build_brick_sprite(x, y));
        }
    }
}

fn build_brick_sprite(x: i8, y: i8) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.5, 0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(0.95, 0.95)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            x as f32, //
            y as f32, 0.,
        )),
        ..default()
    }
}
