use bevy::app::App;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_mod_picking::highlight::Highlight;
use bevy_mod_picking::highlight::HighlightKind::Fixed;
use bevy_mod_picking::PickableBundle;
use bevy_mod_picking::prelude::NoDeselect;
use rand::Rng;
use crate::dev::dev_bullet::BulletPlugin;
use crate::dev::dev_enemy::{BulletTowerPlugin};
use crate::dev::dev_target::{Health, Target, TargetPlugin};
use crate::dev::dev_tower_ui::DevTowerUiPlugin;

pub mod dev_enemy;
pub mod dev_target;
mod dev_bullet;
mod dev_tower_ui;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Ground;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TargetPlugin)
            .add_plugins(BulletTowerPlugin)
            .add_plugins(BulletPlugin)
            .add_plugins(DevTowerUiPlugin)
            .add_systems(Startup, generate_setup);
    }
}

fn generate_setup(mut commands: Commands,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<StandardMaterial>>) {

    //Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    })
        .insert(Ground)
        .insert(Name::new("Ground"));

    //Socket 01
    spawn_random_plates(&mut commands, &mut meshes, &mut materials, 10);

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

fn spawn_random_plates(commands: &mut Commands,
                       meshes: &mut ResMut<Assets<Mesh>>,
                       materials: &mut ResMut<Assets<StandardMaterial>>, count: u8) {
    let default_color = materials.add(Color::srgba(0.3, 0.3, 0.3, 0.2));
    let selected_color = materials.add(Color::srgba(0.8, 0.0, 0.3, 0.8));
    let mut rng = rand::thread_rng();
    let max_value = 10.0;
    let min_value = -10.0;
    //Select Node
    for _ in 0..count {
        let x_rand = rng.gen_range(min_value..max_value);
        let z_rand = rng.gen_range(min_value..max_value);
        commands.spawn(SpatialBundle::from_transform(Transform::from_xyz(x_rand, 0.5, z_rand)))
            .insert(Name::new(format!("Socket-{}", count)))
            .insert(meshes.add(Capsule3d::new(0.2, 0.4)))
            .insert(Highlight {
                hovered: Some(Fixed(selected_color.clone())),
                pressed: Some(Fixed(selected_color.clone())),
                selected: Some(Fixed(selected_color.clone())),
            })
            .insert(default_color.clone())
            .insert(NotShadowCaster)
            .insert(PickableBundle::default())
            .insert(NoDeselect)
            .with_children(|commands| {
                //Plate
                commands.spawn(PbrBundle  {
                    mesh: meshes.add(Cuboid::new(0.5, 0.12, 0.5)),
                    material: materials.add(Color::srgb_u8(100, 100, 110)),
                    transform: Transform::from_xyz(0.0, -0.5, 0.0),
                    ..default()
                });
            });
    }
}