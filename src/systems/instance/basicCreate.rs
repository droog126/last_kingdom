use crate::{
    state::loading::{ImageCenter, TextureAtlasCenter},
    utils::num::y_to_z,
};

use super::{
    animation::{AnimationInfo, AnimationMachine, AnimationValue},
    attack::AttackStorehouseArr,
    collision::{CollisionExcludeFunction, CollisionInput, CollisionResultArr, CollisionShape},
    iType::{player::PlayerAnimationTag, *},
    props::InstanceProps,
    InstanceCollisionTag,
};
use bevy::{prelude::*, sprite::Anchor};

pub fn create_collision(
    commands: &mut Commands,
    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    collisionExcludeFunction: Option<CollisionExcludeFunction>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Entity {
    let collisionId = commands.spawn().id();
    let mut instance = commands.entity(collisionId);
    instance
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(width, height)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, y_to_z(y))),
            ..default()
        })
        // 实例类型判断
        .insert(InstanceTypeValue { value: instanceType })
        .insert(InstanceCampValue { value: instanceCamp })
        // 碰撞相关
        .insert(InstanceCollisionTag)
        .insert(CollisionInput {
            exclude: collisionExcludeFunction,
            receiveId: collisionId,
            shape: CollisionShape { widthHalf: width / 2.0, heightHalf: height / 2.0, pos: Vec2::new(x, y) },
        })
        .insert(CollisionResultArr { arr: vec![] })
        .insert(AttackStorehouseArr { arr: vec![] })
        .insert(Visibility { is_visible: false });

    return collisionId;
}

pub fn create_dyn_collision(
    commands: &mut Commands,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    collisionExcludeFunction: Option<CollisionExcludeFunction>,
) -> Entity {
    let collisionId = create_collision(
        commands,
        instanceType,
        instanceCamp,
        collisionExcludeFunction,
        x,
        y,
        width,
        height,
    );

    let mut instance = commands.entity(collisionId);
    instance.insert(CollisionTypeValue { value: CollisionType::Instance });

    return collisionId;
}

pub fn create_sta_collision(
    commands: &mut Commands,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    collisionExcludeFunction: Option<CollisionExcludeFunction>,
) -> Entity {
    let collisionId = create_collision(
        commands,
        instanceType,
        instanceCamp,
        collisionExcludeFunction,
        x,
        y,
        width,
        height,
    );

    let mut instance = commands.entity(collisionId);
    instance.insert(CollisionTypeValue { value: CollisionType::Static });
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
    let collisionId = create_collision(
        commands,
        instanceType,
        instanceCamp,
        collisionExcludeFunction,
        x,
        y,
        width,
        height,
    );

    let mut instance = commands.entity(collisionId);
    instance.insert(CollisionTypeValue { value: CollisionType::Scope });
    return collisionId;
}

pub fn create_shadow(commands: &mut Commands, imageCenter: &Res<ImageCenter>, width: f32, height: f32) -> Entity {
    let shadowId = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite { anchor: Anchor::Custom(Vec2::new(0.0, -0.1)), ..default() },
            texture: imageCenter.0.get("shadow").unwrap().clone(),
            transform: Transform {
                scale: Vec3::new(width / 10.0, height / 20.0, 0.0),
                translation: Vec3::new(0.0, 0.0, -2.0),
                ..Default::default()
            },
            ..default()
        })
        .id();
    shadowId
}

pub fn create_animation(
    commands: &mut Commands,
    textureAtlasCenter: &Res<TextureAtlasCenter>,
    spriteName: String,
    spriteConfig: &fn(&AnimationValue) -> AnimationInfo,
    x: f32,
    y: f32,
) -> Entity {
    let animationId = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { anchor: Anchor::BottomCenter, ..default() },
            transform: Transform { translation: Vec3::new(x, y, -1.0), ..Default::default() },
            texture_atlas: textureAtlasCenter.0.get(&spriteName).unwrap().clone(),
            ..Default::default()
        })
        .insert(AnimationMachine { value: AnimationValue::Idle, progress: 0.0, config: *spriteConfig })
        .insert(Name::new("animation".to_string()))
        .insert(PlayerAnimationTag)
        .id();
    animationId
}
