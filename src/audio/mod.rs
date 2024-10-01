use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, internal_check);
    }
}

fn internal_check(mut commands: Commands, asset_server: Res<AssetServer>) {
    let settings = PlaybackSettings {
        volume: Volume::new(0.2),
        speed: 1.0,
        mode: PlaybackMode::Loop,
        ..default()
    };

    commands.spawn(AudioBundle {
        source: asset_server.load("sound/DS3M.mp3"),
        settings,
        ..default()
    });
}