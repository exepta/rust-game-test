use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn update_camera_controls(keyboard: Res<ButtonInput<KeyCode>>,
                              mut camera_query: Query<&mut Transform, With<Camera3d>>,
                              time: Res<Time>) {
    let mut camera = camera_query.single_mut();
    let forward = camera.forward().as_vec3();

    let mut left = camera.left().as_vec3();
    left.y = 0.0;
    left = left.normalize();

    let speed :f32 = 3.0;
    let rotate_speed :f32 = 1.5;
    if keyboard.pressed(KeyCode::KeyW) {
        camera.translation += forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        camera.translation += left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        camera.translation -= left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::KeyQ) {
        camera.rotate_axis(Dir3::Y, rotate_speed * time.delta_seconds());
    }

    if keyboard.pressed(KeyCode::KeyE) {
        camera.rotate_axis(Dir3::Y, -rotate_speed * time.delta_seconds());
    }
}