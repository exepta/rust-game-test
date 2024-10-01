use bevy::prelude::*;
use crate::entities::camera::CameraPlugin;
use crate::entities::player::PlayerPlugin;

pub mod player;
mod camera;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayerPlugin)
            .add_plugins(CameraPlugin);
    }
}

#[derive(Component)]
pub struct Speed {
    pub value: f32,
}