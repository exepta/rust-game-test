use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system_display_main_menu);
    }
}

fn system_display_main_menu(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(25.0),
            ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
    })
        .insert(Name::new("Main Menu"))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Mira", TextStyle {font_size: 54.0, color: Color::srgb(0.8, 0.8, 0.8),..default()}),
                ..default()
            });

            include_btn(parent, "Play Game");
            include_btn(parent, "Settings");
            include_btn(parent, "Quit Game");
        });
}

fn include_btn(builder: &mut ChildBuilder, text: &str) {
    builder.spawn(ButtonBundle {
        style: Style {
            width: Val::Percent(50.0),
            height: Val::Px(45.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect {
                top: Val::Px(0.5),
                bottom: Val::Px(0.5),
                left: Val::Px(0.5),
                right: Val::Px(0.5),
            },
            ..default()
        },
        background_color: BackgroundColor::from(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        border_radius: BorderRadius::all(Val::Px(5.0)),
        border_color: BorderColor::from(Color::srgb(0.7, 0.7, 0.7)),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(text.to_string(), TextStyle::default()),
            ..default()
        });
    });
}