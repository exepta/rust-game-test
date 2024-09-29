mod camera;
mod debug;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::camera::spawn_camera;
use crate::debug::DebugPlugin;

pub const WIDTH : f32 = 1270.0;
pub const HEIGHT : f32 = 720.0;

fn main() {
    initialize();
}

fn initialize() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Mira Test 3D".to_string(),
            resizable: false,
            resolution: WindowResolution::new(WIDTH, HEIGHT),
            ..default()
        }),
        ..default()
    }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DebugPlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}