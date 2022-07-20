use crate::utils::num::y_to_z;

use super::{
    attack::AttackStorehouseArr,
    collision::{CollisionExcludeFunction, CollisionInput, CollisionResultArr, CollisionShape},
    iType::*,
    props::InstanceProps,
    InstanceCollisionTag,
};
use bevy::prelude::*;

pub fn create_instance_collision(
    commands: &mut Commands,
    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    collisionExcludeFunction: Option<CollisionExcludeFunction>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    instanceProps: InstanceProps,
) -> Entity {
    let collisionId = commands.spawn().id();
    commands
        .entity(collisionId)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, y_to_z(y))),
            ..default()
        })
        .insert(Name::new("collision"))
        // 实例类型判断
        .insert(InstanceTypeValue { value: instanceType })
        .insert(InstanceCampValue { value: instanceCamp })
        .insert(CollisionTypeValue { value: CollisionType::Instance })
        // 碰撞相关
        .insert(InstanceCollisionTag)
        .insert(CollisionInput {
            exclude: collisionExcludeFunction,
            receiveId: collisionId,
            shape: CollisionShape { widthHalf: width / 2.0, heightHalf: height / 2.0, pos: Vec2::new(x, y) },
        })
        .insert(CollisionResultArr { arr: vec![] })
        .insert(AttackStorehouseArr { arr: vec![] })
        //
        .insert(instanceProps)
        .insert(Visibility { is_visible: false });

    return collisionId;
}

pub fn create_sta_collision(commands: &mut Commands, x: f32, y: f32, width: f32, height: f32) -> Entity {
    let collisionId = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.4, 0.4, 0.4),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, y_to_z(y))),
            ..default()
        })
        .insert(Name::new("staCollision"))
        .id();

    return collisionId;
}

pub fn create_scope_collision(
    commands: &mut Commands,
    parentId: Entity,
    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    collisionExcludeFunction: Option<CollisionExcludeFunction>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Entity {
    let collisionId = commands
        .spawn()
        .insert(GlobalTransform { ..default() })
        .insert(Transform { translation: Vec3::new(x, y, 100.0 - y / 10000.0), ..default() })
        .id();

    commands
        .entity(collisionId)
        .insert(InstanceTypeValue { value: instanceType })
        .insert(InstanceCampValue { value: instanceCamp })
        .insert(CollisionTypeValue { value: CollisionType::Scope })
        .insert(CollisionInput {
            exclude: collisionExcludeFunction,
            receiveId: parentId,
            shape: CollisionShape { widthHalf: width / 2.0, heightHalf: height / 2.0, pos: Vec2::new(x, y) },
        })
        .insert(CollisionResultArr { arr: vec![] });

    return collisionId;
}
