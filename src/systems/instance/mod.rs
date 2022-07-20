use bevy::prelude::*;

pub mod animation;
pub mod attack;
pub mod basicCreate;
pub mod collision;
pub mod factory;
pub mod iType;
pub mod props;

// 标识实体
#[derive(Component)]
pub struct InstanceCollisionTag;

// 小系统
pub fn z_depth_step(time: Res<Time>, mut query: Query<&mut Transform, With<InstanceCollisionTag>>) {
    for (mut transform) in query.iter_mut() {
        transform.translation.z = 100.0 - transform.translation.y / 10000.0
    }
}
