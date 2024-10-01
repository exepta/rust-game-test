use bevy::ecs::component::StorageType;
use bevy::prelude::*;
use crate::audio::AudioPlugin;
use crate::entities::EntitiesPlugin;
use crate::ui::UIPlugin;
use crate::world::TerrainPlugin;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        info!("Loaded Manager plugin");
        app
            .add_plugins(TerrainPlugin)
            .add_plugins(EntitiesPlugin)
            .add_plugins(UIPlugin)
            .add_plugins(AudioPlugin);
    }
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct MainMenu;

impl ComputedStates for MainMenu {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::MainMenu => Some(MainMenu),
            _ => None,
        }
    }
}


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Playing;

impl ComputedStates for Playing {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::Playing => Some(Playing),
            _ => None,
        }
    }
}


