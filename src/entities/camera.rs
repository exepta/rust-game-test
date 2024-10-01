use bevy::prelude::*;
use bevy_third_person_camera::{ThirdPersonCamera, Zoom};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, system_spawn_player_camera);
    }
}

fn system_spawn_player_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-7.1, 6.8, 14.2).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(ThirdPersonCamera {
        sensitivity: Vec2::new(3.0, 3.0),
        zoom: Zoom::new(2.0, 7.5),
        ..default()
    });
}