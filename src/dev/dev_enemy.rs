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
                             mut towers: Query<(Entity, &mut TrapTower, &GlobalTransform)>,
                             targets: Query<&GlobalTransform, With<Target>>,
                             time: Res<Time>) {
    for (tower_en, mut tower, transform) in towers.iter_mut() {
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
                commands.entity(tower_en).with_children(|commands| {
                    //Bullet
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Cuboid::new(0.05, 0.05, 0.05)),
                        material: materials.add(Color::srgb_u8(255, 0, 0)),
                        transform: Transform::from_translation(tower.bullet_offset),
                        ..default()
                    }).insert(Lifetime {
                        timer: Timer::from_seconds(3., TimerMode::Once),
                    }).insert(Bullet {
                        direction,
                        speed: 8.0,
                    }).insert(Name::new("Bullet"));
                });
            }
        }
    }
}

pub fn build_tower(mut commands: Commands,
                   selections: Query<(Entity, &PickSelection, &Transform)>,
                   mut meshes: ResMut<Assets<Mesh>>,
                   mut materials: ResMut<Assets<StandardMaterial>>,
                   mouse: Res<ButtonInput<MouseButton>>) {
    if mouse.just_pressed(MouseButton::Left) {
        for (entity, selection, transform) in selections.iter() {
            if selection.is_selected {
                commands.entity(entity).despawn_recursive();
                spawn_normal_tower(&mut commands, &mut meshes, &mut materials, transform.translation);
            }
        }
    }
}

pub fn spawn_normal_tower(commands: &mut Commands,
                          meshes: &mut ResMut<Assets<Mesh>>,
                          materials: &mut ResMut<Assets<StandardMaterial>>,
                          position: Vec3) -> Entity {
    commands.spawn(PbrBundle  {
        mesh: meshes.add(Cuboid::new(0.5, 1.0, 0.5)),
        material: materials.add(Color::srgb_u8(30, 30, 40)),
        transform: Transform::from_translation(position),
        ..default()
    })
        .insert(Name::new("TrapTower"))
        .insert(TrapTower {
            shooting_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0.5, 0.6)
        }).id()
}