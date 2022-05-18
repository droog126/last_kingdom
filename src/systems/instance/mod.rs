pub mod shadow;

use bevy::prelude::*;

use crate::{
    instance::{
        player::{player_create, player_step},
        snake::snake_step,
    },
    state::GameState,
};

use self::shadow::ShadowPlugin;

pub struct InstancePlugin;
impl Plugin for InstancePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShadowPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(player_create))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player_step)
                    .with_system(snake_step)
                    .with_system(z_depth_step),
            );
    }
}

#[derive(Component)]
pub struct InstanceCollisionTag;

pub fn z_depth_step(time: Res<Time>, mut query: Query<&mut Transform, With<InstanceCollisionTag>>) {
    for (mut transform) in query.iter_mut() {
        transform.translation.z = 100.0 - transform.translation.y / 10000.0
    }
}
