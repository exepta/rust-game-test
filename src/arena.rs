use bevy::asset::Assets;
use bevy::color::Color;
use bevy::color::palettes::css::DARK_GRAY;
use bevy::math::Vec3;
use bevy::prelude::{default, ColorMaterial, Commands, Mesh, Query, Rectangle, ResMut, Transform, Window, With};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

pub fn update_scene(mut commands: Commands,
          windows: Query<&Window, With<PrimaryWindow>>,
          mut meshes: ResMut<Assets<Mesh>>,
          mut materials: ResMut<Assets<ColorMaterial>>) {
    let area_height = 50.0;
    let mut width = 10.0;
    let mut height = 10.0;

    if let Ok(window) = windows.get_single() {
        width = window.width();
        height = window.height();
    }

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(width, area_height)).into(),
        material: materials.add(Color::from(DARK_GRAY)),
        transform: Transform {
            translation: Vec3::new(0.0, -(height / 2.0) + (area_height / 2.0), 0.0),
            ..default()
        },
        ..default()
    });
}