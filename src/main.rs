mod arena;
mod entities;

use bevy::prelude::*;
use crate::arena::update_scene;

fn main() {
    initialize();
}

fn initialize() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "2D Test Game".to_string(),
            resolution: (800.0, 600.0).into(),
            ..default()
        }),
        ..default()
    }))
        .add_systems(Startup, (setup, update_scene))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}