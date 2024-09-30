use bevy::app::App;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_mod_picking::highlight::Highlight;
use bevy_mod_picking::highlight::HighlightKind::Fixed;
use bevy_mod_picking::PickableBundle;
use crate::dev::dev_bullet::BulletPlugin;
use crate::dev::dev_enemy::{BulletTowerPlugin};
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
        mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    })
        .insert(Ground)
        .insert(Name::new("Ground"));

    let default_color = materials.add(Color::srgba(0.3, 0.3, 0.3, 0.2));
    let selected_color = materials.add(Color::srgba(0.8, 0.0, 0.3, 0.8));

    //Socket
    commands.spawn(SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.5, 0.0)))
        .insert(Name::new("Socket"))
        .insert(meshes.add(Capsule3d::new(0.2, 0.4)))
        .insert(Highlight {
            hovered: Some(Fixed(selected_color.clone())),
            pressed: Some(Fixed(selected_color.clone())),
            selected: Some(Fixed(selected_color.clone())),
        })
        .insert(default_color.clone())
        .insert(NotShadowCaster)
        .insert(PickableBundle::default())
        .with_children(|commands| {
            //Tower
            commands.spawn(PbrBundle  {
                mesh: meshes.add(Cuboid::new(0.5, 0.12, 0.5)),
                material: materials.add(Color::srgb_u8(100, 100, 110)),
                transform: Transform::from_xyz(0.0, -0.5, 0.0),
                ..default()
            });
        });

    //Player (Placeholder)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.3, 0.3, 0.3)),
        material: materials.add(Color::srgb_u8(0, 0, 180)),
        transform: Transform::from_xyz(- 2.0, 0.2, 1.5),
        ..default()
    })
        .insert(Target{speed: 0.25})
        .insert(Health{value: 650})
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