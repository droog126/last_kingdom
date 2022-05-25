use crate::state::loading::SpriteCenter;
use crate::systems::collision::{
    CollisionExcludeFunction, CollisionInput, CollisionResultArr, CollisionShape,
};
use crate::systems::instance::shadow::ShadowAsset;
use crate::systems::instance::InstanceCollisionTag;
use crate::systems::stateMachine::{AnimationState, StateInfo, StateMachine};
use crate::utils::num::y_to_z;
use bevy::math::Vec2;
use bevy::prelude::*;

use super::{
    CollisionType, CollisionTypeValue, InstanceCamp, InstanceCampValue, InstanceType,
    InstanceTypeValue,
};

pub fn create_instance_collision(
    commands: &mut Commands,
    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    collisionExcludeFunction: Option<CollisionExcludeFunction>,

    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Entity {
    let collisionId = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, y_to_z(y))),
            ..default()
        })
        .insert(Name::new("collision"))
        .insert(Visibility { is_visible: false })
        .insert(InstanceCollisionTag)
        .id();

    commands
        .entity(collisionId)
        .insert(InstanceTypeValue {
            value: instanceType,
        })
        .insert(InstanceCampValue {
            value: instanceCamp,
        })
        .insert(CollisionTypeValue {
            value: CollisionType::Instance,
        })
        .insert(CollisionInput {
            exclude: collisionExcludeFunction,
            receiveId: collisionId,
            shape: CollisionShape {
                widthHalf: width / 2.0,
                heightHalf: height / 2.0,
                pos: Vec2::new(x, y),
            },
        })
        .insert(CollisionResultArr { arr: vec![] });

    return collisionId;
}

pub fn create_sta_collision(
    commands: &mut Commands,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Entity {
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
        // .insert(Visibility { is_visible: false })
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
        .insert(Transform {
            translation: Vec3::new(x, y, 100.0 - y / 10000.0),
            ..default()
        })
        .id();

    commands
        .entity(collisionId)
        .insert(InstanceTypeValue {
            value: instanceType,
        })
        .insert(InstanceCampValue {
            value: instanceCamp,
        })
        .insert(CollisionTypeValue {
            value: CollisionType::Scope,
        })
        .insert(CollisionInput {
            exclude: collisionExcludeFunction,
            receiveId: parentId,
            shape: CollisionShape {
                widthHalf: width / 2.0,
                heightHalf: height / 2.0,
                pos: Vec2::new(x, y),
            },
        })
        .insert(CollisionResultArr { arr: vec![] });

    return collisionId;
}

pub fn create_attack_box<T>(
    shadowImage: Handle<Image>,
    textureAtlas: Handle<TextureAtlas>,
    getSpriteIndex: fn(&AnimationState) -> StateInfo,
    commands: &mut Commands,
    name: &str,

    // parentId: Entity,
    tag: T,

    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    collisionExcludeFunction: Option<CollisionExcludeFunction>,

    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Entity {
    // 阴影实体

    let shadowId = commands
        .spawn_bundle(SpriteBundle {
            texture: shadowImage,
            transform: Transform {
                scale: Vec3::new(1.0, 0.5, 0.0),
                ..default()
            },
            ..default()
        })
        .id();
    let animationId = commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(4.0, 14.0, 10.0),
                ..Default::default()
            },
            texture_atlas: textureAtlas,
            ..Default::default()
        })
        .insert(AnimationState(StateMachine::Idle, 1.0, getSpriteIndex))
        .insert(Name::new(name.to_string()))
        .id();

    let instanceId = commands.spawn().id();
    commands
        .entity(instanceId)
        .insert(InstanceTypeValue {
            value: instanceType,
        })
        .insert(InstanceCampValue {
            value: instanceCamp,
        })
        .insert(CollisionTypeValue {
            value: CollisionType::Scope,
        })
        .insert(CollisionInput {
            exclude: collisionExcludeFunction,
            receiveId: instanceId,
            shape: CollisionShape {
                widthHalf: width / 2.0,
                heightHalf: height / 2.0,
                pos: Vec2::new(x, y),
            },
        })
        .insert(CollisionResultArr { arr: vec![] })
        .id();
    commands
        .entity(instanceId)
        .push_children(&[animationId, shadowId]);
    instanceId
}

pub fn create_attack_free_box<T>(
    commands: &mut Commands,
    parentId: Entity,
    tag: T,

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
        .insert(Transform {
            translation: Vec3::new(x, y, 100.0 - y / 10000.0),
            ..default()
        })
        .id();

    commands
        .entity(collisionId)
        .insert(InstanceTypeValue {
            value: instanceType,
        })
        .insert(InstanceCampValue {
            value: instanceCamp,
        })
        .insert(CollisionTypeValue {
            value: CollisionType::Scope,
        })
        .insert(CollisionInput {
            exclude: collisionExcludeFunction,
            receiveId: parentId,
            shape: CollisionShape {
                widthHalf: width / 2.0,
                heightHalf: height / 2.0,
                pos: Vec2::new(x, y),
            },
        })
        .insert(CollisionResultArr { arr: vec![] });

    return collisionId;
}
