use bevy::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        info!("Loading world");
        app.add_systems(Startup, (system_floor_setup, system_floor_light));
    }
}

fn system_floor_setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
        material: materials.add(Color::srgb(0.3, 0.3, 0.4)),
        ..default()
    });
}

fn system_floor_light(mut commands: Commands, _materials: ResMut<Assets<StandardMaterial>>, _meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0 , 0.0),
        ..default()
    });
}