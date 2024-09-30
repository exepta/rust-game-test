use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: u32,
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_systems(Update, (update_target_movement, update_target_life_state));
    }
}

// Todo: make player struct and implements movement. Now let the time change the position...
pub fn update_target_movement(mut target: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in target.iter_mut() {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

pub fn update_target_life_state(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (target, health) in targets.iter() {
        if health.value <= 0 {
            commands.entity(target).despawn_recursive();
        }
    }
}