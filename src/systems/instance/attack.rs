use crate::utils::num::y_to_z;

use super::collision::{CollisionExcludeFunction, CollisionInput, CollisionResultArr, CollisionShape};
use super::iType::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct AttackBoxTag;

#[derive(Component, Debug)]
pub struct AttackStorehouseArr {
    pub arr: Vec<AttackEvent>,
}

#[derive(Debug, Component, Clone)]
pub struct AttackEvent {
    pub id: Entity,
    pub damage: f32,
    pub nextTime: i32,
    pub repelData: Option<RepelData>,
}
pub struct AttackEventPart {
    pub damage: f32,
    pub nextTime: i32,
    pub repelData: Option<RepelData>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct RepelData {
    pub dif: Vec3,
    pub timeLen: i32,
}

// 创建后计算所有范围内的实体后立即销毁
// todo:创建后一定时间内销毁，或者碰到东西立即销毁 攻击盒子
pub fn attack_event_distribution_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CollisionResultArr, &AttackEvent), With<AttackBoxTag>>,
    mut attackQuery: Query<&mut AttackStorehouseArr, Without<AttackBoxTag>>,
) {
    for (entity, mut collisionResultArr, attackEvent) in query.iter_mut() {
        for item in collisionResultArr.arr.iter() {
            if let Ok(mut attackStorehouseArr) = attackQuery.get_mut(item.id) {
                attackStorehouseArr.arr.push(attackEvent.clone());
            }
        }
        // 释放后立即销毁
        collisionResultArr.arr.clear();
        commands.entity(entity).despawn();
    }
}

// 提交到实例创建工厂里
pub fn create_instance(commands: &mut Commands, imageHandle: Handle<Image>, x: f32, y: f32) {}

pub fn create_attack_box(
    commands: &mut Commands,
    imageHandle: Handle<Image>,
    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    collisionExcludeFunction: Option<CollisionExcludeFunction>,
    attackEventPart: AttackEventPart,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Entity {
    let collisionId = commands
        .spawn()
        .insert(GlobalTransform { ..default() })
        .insert(Transform { translation: Vec3::new(x, y, y_to_z(y)), ..default() })
        .insert(Visibility { ..default() })
        .insert(imageHandle)
        .insert(Sprite { ..default() })
        .id();

    commands
        .entity(collisionId)
        .insert(InstanceTypeValue { value: instanceType })
        .insert(InstanceCampValue { value: instanceCamp })
        .insert(CollisionTypeValue { value: CollisionType::Scope })
        .insert(CollisionInput {
            exclude: collisionExcludeFunction,
            receiveId: collisionId,
            shape: CollisionShape { widthHalf: width / 2.0, heightHalf: height / 2.0, pos: Vec2::new(x, y) },
        })
        .insert(CollisionResultArr { arr: vec![] })
        .insert(AttackBoxTag)
        .insert(AttackEvent {
            id: collisionId,
            damage: attackEventPart.damage,
            nextTime: attackEventPart.nextTime,
            repelData: attackEventPart.repelData,
        });

    return collisionId;
}
