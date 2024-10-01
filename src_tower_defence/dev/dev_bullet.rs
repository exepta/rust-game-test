use bevy::prelude::*;
use crate::dev::dev_target::{Health, Target};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub(crate) direction: Vec3,
    pub(crate) speed: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub(crate) timer: Timer,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bullet>()
            .register_type::<Lifetime>()
            .add_systems(Update, (update_bullets_pos, update_bullet_despairing, update_bullets_collision));
    }
}

pub fn update_bullet_despairing(mut commands: Commands,
                                mut bullets: Query<(Entity, &mut Lifetime)>,
                                time: Res<Time>) {
    for (entity, mut lifetime) in bullets.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn update_bullets_pos(mut bullets: Query<(&Bullet, &mut Transform)>,
                          time: Res<Time>) {
    for (bullet, mut transform) in bullets.iter_mut() {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

pub fn update_bullets_collision(mut commands: Commands,
                                bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
                                mut targets: Query<(&mut Health, &Transform), With<Target>>) {
    for (bullet, bullet_transform) in bullets.iter() {
        for (mut health, target_transform) in targets.iter_mut() {
            if Vec3::distance(bullet_transform.translation(), target_transform.translation) < 0.2 {
                commands.entity(bullet).despawn_recursive();
                health.value -= 1;
                break;
            }
        }
    }
}