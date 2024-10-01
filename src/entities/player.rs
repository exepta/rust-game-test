use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;
use crate::entities::Speed;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, system_spawn_player)
            .add_systems(Update, update_player_movement);
    }
}

#[derive(Component)]
pub struct Player;

fn update_player_movement(keyboard: Res<ButtonInput<KeyCode>>, time: Res<Time>,
                          mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
                          camera_query: Query<&Transform, (With<Camera>, Without<Player>)>) {
    for (mut player_transform, speed) in player_query.iter_mut() {
        let cam = camera_query.get_single().unwrap_or_else(|_ose| {
            unreachable!("Camera does not exist or is invalid");
        });

        let mut direction = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) {
            direction += cam.forward().as_vec3();
        }

        if keyboard.pressed(KeyCode::KeyS) {
            direction += cam.back().as_vec3();
        }

        if keyboard.pressed(KeyCode::KeyA) {
            direction += cam.left().as_vec3();
        }

        if keyboard.pressed(KeyCode::KeyD) {
            direction += cam.right().as_vec3();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * speed.value * time.delta_seconds();
        player_transform.translation += movement;

        if direction.length_squared() > 0.0 {
            let _ = player_transform.look_to(direction, Vec3::Y);
        }
    }
}

fn system_spawn_player(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: assets_server.load("models/player.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.4, 0.0),
        ..default()
    })
        .insert(Name::from("Player"))
        .insert(Player)
        .insert(Speed {
            value: 3.0,
        })
        .insert(ThirdPersonCameraTarget);
}