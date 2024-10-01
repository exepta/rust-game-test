use bevy::math::FloatOrd;
use bevy::prelude::*;
use bevy_mod_picking::prelude::PickSelection;
use crate::dev::dev_bullet::{Bullet, Lifetime};
use crate::dev::dev_target::Target;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TrapTower {
    pub(crate) shooting_timer: Timer,
    pub(crate) bullet_offset: Vec3,
}

#[derive(Component, Clone, Copy, Debug)]
pub enum TowerType {
    Normal,
    Laser,
    Flamethrower,
}

impl TowerType {
    fn get_tower(&self, materials: &mut ResMut<Assets<StandardMaterial>>) -> (Handle<StandardMaterial>, TrapTower) {
        match self {
            TowerType::Normal => (
                materials.add(Color::srgb(0.5, 0.5, 0.8)),
                TrapTower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.5, 0.0),
                },
            ),
            TowerType::Laser => (
                materials.add(Color::srgb(0.0, 0.5, 0.8)),
                TrapTower {
                    shooting_timer: Timer::from_seconds(0.001, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.5, 0.0),
                },
            ),
            TowerType::Flamethrower => (
                materials.add(Color::srgb(0.8, 0.0, 0.7)),
                TrapTower {
                    shooting_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.5, 0.0),
                },
            ),
        }
    }

    fn get_bullet(&self, direction: Vec3, materials: &mut ResMut<Assets<StandardMaterial>>) -> (Handle<StandardMaterial>, Bullet) {
        match self {
            TowerType::Normal => (
                materials.add(Color::srgb(0.0, 0.5, 1.0)),
                Bullet {
                    direction,
                    speed: 2.0,
                }
            ),
            TowerType::Laser => (
                materials.add(Color::srgb(0.0, 1.0, 0.4)),
                Bullet {
                    direction,
                    speed: 10.0,
                }
            ),
            TowerType::Flamethrower => (
                materials.add(Color::srgb(1.0, 0.2, 0.0)),
                Bullet {
                    direction,
                    speed: 1.5,
                }
            ),
        }
    }
}

pub struct BulletTowerPlugin;

impl Plugin for BulletTowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TrapTower>()
            .add_systems(Update, (update_tower_shooting, build_tower));
    }
}

pub fn update_tower_shooting(mut commands: Commands,
                             mut meshes: ResMut<Assets<Mesh>>,
                             mut materials: ResMut<Assets<StandardMaterial>>,
                             mut towers: Query<(Entity, &mut TrapTower, &TowerType, &GlobalTransform)>,
                             targets: Query<&GlobalTransform, With<Target>>,
                             time: Res<Time>) {
    for (tower_en, mut tower, tower_type, transform) in towers.iter_mut() {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_bullet = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .min_by_key(|target_transform | {
                    FloatOrd(Vec3::distance(target_transform.translation(), spawn_bullet))
                })
                .map(|nearest_target| nearest_target.translation() - spawn_bullet);

            if let Some(direction) = direction {
                let (color, bullet) = tower_type.get_bullet(direction, &mut materials);
                commands.entity(tower_en).with_children(|commands| {
                    //Bullet
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Cuboid::new(0.05, 0.05, 0.05)),
                        material: color,
                        transform: Transform::from_translation(tower.bullet_offset),
                        ..default()
                    }).insert(Lifetime {
                        timer: Timer::from_seconds(3., TimerMode::Once),
                    })
                        .insert(bullet)
                        .insert(Name::new("Bullet"));
                });
            }
        }
    }
}

pub fn build_tower(mut commands: Commands,
                   selections: Query<(Entity, &PickSelection, &Transform)>,
                   interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>,
                   mut meshes: ResMut<Assets<Mesh>>,
                   mut materials: ResMut<Assets<StandardMaterial>>) {

    for (interaction, tower_type) in interaction.iter() {
        if matches!(interaction, Interaction::Pressed) {
            for (entity, selection, transform) in selections.iter() {
                if selection.is_selected {
                    commands.entity(entity).despawn_recursive();
                    spawn_tower(&mut commands, &mut meshes, &mut materials, transform.translation, *tower_type);
                }
            }
        }
    }
}

pub fn spawn_tower(commands: &mut Commands,
                   meshes: &mut ResMut<Assets<Mesh>>,
                   materials: &mut ResMut<Assets<StandardMaterial>>,
                   position: Vec3,
                   tower_type: TowerType) -> Entity {
    info!("Spawning tower {:?}", tower_type);
    let (color, tower) = tower_type.get_tower(materials);

    commands.spawn(SpatialBundle::from_transform(Transform::from_translation(position)))
        .insert(Name::new(format!("{:?}_Tower", tower_type)))
        .insert(tower_type)
        .insert(tower)
        .with_children(|commands| {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.4, 1.5, 0.4)),
                material: color,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
        })
        .id()
}