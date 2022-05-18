use crate::systems::collision::{CollisionInput, CollisionResultArr, CollisionShape};
use crate::systems::instance::InstanceCollisionTag;
use crate::utils::num::y_to_z;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    CollisionType, CollisionTypeValue, InstanceCamp, InstanceCampValue, InstanceType,
    InstanceTypeValue,
};

pub fn create_instance_collision(
    commands: &mut Commands,
    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    collisionExcludeFunction: Option<fn(&InstanceType, &CollisionType, &InstanceCamp) -> bool>,

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
    // commands
    //     .entity(collisionId)
    //     // .insert(InstanceTypeValue {
    //     //     value: instanceType,
    //     // })
    //     // .insert(InstanceCampValue {
    //     //     value: instanceCamp,
    //     // })
    //     .insert(CollisionTypeValue {
    //         value: CollisionType::Instance,
    //     })
    //     .insert(CollisionInput {
    //         exclude: ,
    //         receiveId: collisionId,
    //         shape: CollisionShape {
    //             widthHalf: width / 2.0,
    //             heightHalf: height / 2.0,
    //             pos: Vec2::new(x, y),
    //         },
    //     })
    //     .insert(CollisionResultArr { arr: vec![] });

    return collisionId;
}

pub fn create_scope_collision(
    commands: &mut Commands,
    parentId: Entity,

    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    collisionExcludeFunction: Option<fn(&InstanceType, &CollisionType, &InstanceCamp) -> bool>,

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
