use bevy::app::App;
use bevy::prelude::*;
use crate::dev::dev_bullet::BulletPlugin;
use crate::dev::dev_enemy::{BulletTowerPlugin, TrapTower};
use crate::dev::dev_target::{Health, Target, TargetPlugin};

pub mod dev_enemy;
pub mod dev_target;
mod dev_bullet;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Ground;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TargetPlugin)
            .add_plugins(BulletTowerPlugin)
            .add_plugins(BulletPlugin)
            .add_systems(Startup, generate_setup);
    }
}

fn generate_setup(mut commands: Commands,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<StandardMaterial>>) {

    //Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    })
        .insert(Ground)
        .insert(Name::new("Ground"));

    //Tower
    commands.spawn(PbrBundle  {
        mesh: meshes.add(Cuboid::new(0.5, 1.0, 0.5)),
        material: materials.add(Color::srgb_u8(30, 30, 40)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
        .insert(Name::new("TrapTower"))
        .insert(TrapTower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0.5, 0.6)
        });

    //Player (Placeholder)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.3, 0.3, 0.3)),
        material: materials.add(Color::srgb_u8(0, 0, 180)),
        transform: Transform::from_xyz(- 2.0, 0.2, 1.5),
        ..default()
    })
        .insert(Target{speed: 0.3})
        .insert(Health{value: 5})
        .insert(Name::new("Debug Player"));

    //Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}