mod manager;
mod world;
mod entities;
mod audio;
mod ui;

use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use crate::manager::*;

pub const WIDTH : f32 = 1920.0;
pub const HEIGHT : f32 = 1080.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.55, 0.55, 0.65)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mira 3D Alpha Test".to_string(),
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_computed_state::<MainMenu>()
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(ManagerPlugin)
        .run();
}