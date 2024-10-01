use bevy::app::App;
use bevy::color::Color;
use bevy::core::Name;
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::ui::JustifyContent;
use bevy_mod_picking::prelude::{NoDeselect, PickSelection};
use crate::dev::dev_enemy::TowerType;

#[derive(Component)]
pub struct TowerUIRoot;

pub struct DevTowerUiPlugin;

impl Plugin for DevTowerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_tower_button_clicked, update_create_ui_on_selection));
    }
}

pub fn update_create_ui_on_selection(mut commands: Commands,
                                     selections: Query<&PickSelection>,
                                     root: Query<Entity, With<TowerUIRoot>>) {
    let at_least_one_selected = selections.iter().any(|selection| selection.is_selected);
    match root.get_single() {
        Ok(root) => {
            if !at_least_one_selected {
                commands.entity(root).despawn_recursive();
            }
        }
        Err(QuerySingleError::NoEntities( .. )) => {
            if at_least_one_selected {
                create_ui(&mut commands);
            }
        }
        _ => unreachable!("Too many roots!"),
    }
}

pub fn create_ui(commands: &mut Commands) {
    let towers = [TowerType::Normal, TowerType::Laser, TowerType::Flamethrower];
    let colors = [
        Color::srgb(0.5, 0.5, 0.8),
        Color::srgb(0.0, 0.5, 0.8),
        Color::srgb(0.8, 0.0, 0.7)
    ];

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    })
        .insert(Name::new("DevTowerUI"))
        .insert(TowerUIRoot)
        .with_children(|commands| {
            for i in 0.. 3 {
                commands.spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(7.5 * 9.0 / 16.0),
                        height: Val::Percent(7.5),
                        align_self: AlignSelf::FlexEnd,
                        margin: UiRect::all(Val::Percent(2.0)),
                        ..default()
                    },
                    background_color: colors[i].into(),
                    ..default()
                })
                    .insert(towers[i])
                    .insert(NoDeselect)
                    .insert(Name::new(format!("Tower-{}", i)));
            }
        });
}

pub fn update_tower_button_clicked(interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>) {
    for (interaction, tower_type) in interaction.iter() {
        if matches!(interaction, Interaction::Pressed) {
            info!("Clicked tower type: {:?}", tower_type);
        }
    }
}