use crate::state::loading::SpriteCenter;
use crate::systems::collision::{CollisionBot, CollisionID};
use crate::systems::debug::DebugStatus;
use crate::systems::instance::shadow::ShadowAsset;
use crate::systems::instance::InstanceCollisionTag;
use crate::systems::stateMachine::{InsState, StateChangeEvt, StateInfo, StateMachine};
use bevy::utils::HashMap;
use bevy_prototype_lyon::prelude::*;

use bevy::prelude::*;

use super::utils::{create_instance_collision, create_scope_collision};
use super::{InstanceCamp, InstanceCategory, InstanceType};

#[derive(Component)]
pub struct SnakeTag;

#[derive(Component, Debug)]
pub struct SnakeProps {
    pub spd: f32,
}
#[derive(Component)]
pub struct SnakeCollisionTag;

#[derive(Component, Debug)]
pub struct SnakeScopeCollisionTag;

pub struct SnakeAi {
    target: Option<Entity>,
}

fn getSnakeSprite(insState: &InsState) -> StateInfo {
    match (insState.0) {
        StateMachine::Idle => StateInfo {
            startIndex: 0,
            endIndex: 7,
            spriteName: "snake".to_string(),
        },
        StateMachine::Walk => StateInfo {
            startIndex: 8,
            endIndex: 15,
            spriteName: "snake".to_string(),
        },
        _ => StateInfo {
            startIndex: 0,
            endIndex: 0,
            spriteName: "snake".to_string(),
        },
    }
}

pub fn snake_create_raw(
    mut commands: &mut Commands,
    mut spriteCenter: ResMut<SpriteCenter>,
    shadowHandle: Res<ShadowAsset>,
    x: f32,
    y: f32,
) {
    // 阴影实体
    let shadowId = commands
        .spawn_bundle(SpriteBundle {
            texture: shadowHandle.clone(),
            transform: Transform {
                scale: Vec3::new(1.0, 0.5, 0.0),
                ..default()
            },
            ..default()
        })
        .id();

    // 人物实体
    let instanceId = commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(4.0, 14.0, 10.0),
                ..Default::default()
            },
            texture_atlas: spriteCenter.0.get("snake").unwrap().clone(),
            ..Default::default()
        })
        .insert(SnakeProps { spd: 200.0 })
        .insert(InsState(StateMachine::Idle, getSnakeSprite))
        .insert(Name::new("snake".to_string()))
        .insert(SnakeTag)
        .id();

    let collisionId = create_instance_collision(&mut commands, x, y, 20.0, 10.0);
    let scopeCollisionId =
        create_scope_collision(&mut commands, collisionId, 0.0, 0.0, 100.0, 100.0);

    // player后置添加
    commands.entity(instanceId).insert(CollisionID(collisionId));

    // collision后置添加
    commands
        .entity(collisionId)
        .insert(Name::new("snakeCollision"))
        .insert(SnakeCollisionTag)
        .insert(InstanceCategory {
            type_: InstanceType::Snake,
            camp: InstanceCamp::Hostile,
        })
        .push_children(&[instanceId, shadowId, scopeCollisionId]);
    commands
        .entity(scopeCollisionId)
        .insert(SnakeScopeCollisionTag);
}

// 运行限制条件，snake确实存在  可能需要一张表来维护
pub fn snake_step(
    time: Res<Time>,
    mut changeStateSend: EventWriter<StateChangeEvt>,
    debugStatus: Res<DebugStatus>,
    mut set: ParamSet<(
        Query<&mut CollisionBot, With<SnakeScopeCollisionTag>>,
        Query<(&Transform, &InstanceCategory), With<InstanceCollisionTag>>,
        Query<(&mut Transform), With<SnakeCollisionTag>>,
    )>,
) {
    let mut query = set.p0();
    let mut snakeScopeOtherMap = HashMap::new();
    let mut snakeOtherInfoMap = HashMap::new();

    for mut collisionBot in query.iter_mut() {
        match &mut collisionBot.collisionInner {
            crate::systems::collision::CollisionInner::Scope { other, parentId } => {
                snakeScopeOtherMap.insert(parentId.clone(), other.clone());
                other.clear();
            }
            _ => {}
        }
    }

    // 这里是不是可以过滤一下?
    let mut query = set.p1();
    for (entity, OtherEntities) in snakeScopeOtherMap {
        let mut otherInfos = vec![];
        for (otherEntity) in OtherEntities {
            if let Ok((transform, instanceCategory)) = query.get(otherEntity) {
                // otherInfo.push((*transform, instanceCategory.clone()));
                match instanceCategory.camp {
                    InstanceCamp::Neutral => {
                        otherInfos.push((*transform, instanceCategory.clone()));
                    }
                    InstanceCamp::Hostile => {}
                    InstanceCamp::Friendly => {
                        otherInfos.push((*transform, instanceCategory.clone()));
                    }
                    InstanceCamp::Team { team_id } => {
                        otherInfos.push((*transform, instanceCategory.clone()));
                    }
                }
            }
        }
        // snakeOtherInfoMap.insert(entity, otherInfo);
        // 这里我只想第一个
        snakeOtherInfoMap.insert(entity, otherInfos);
    }

    // instanceId : [otherInfo,otherInfo,otherInfo]
    let mut query = set.p2();
    for (entity, otherInfos) in snakeOtherInfoMap {
        let len = otherInfos.len();
        if len >= 1 {
            let (targetTransform, _) = &otherInfos[0];
            if let Ok(mut instanceTransform) = query.get_mut(entity) {
                let mut diff = targetTransform.translation - instanceTransform.translation;
                diff = diff.normalize();
                instanceTransform.translation += diff * time.delta_seconds() * 100.0;
            }
        }
    }
}

pub fn snake_collisionScope_event(mut query: Query<(&mut CollisionBot, &Transform)>) {
    // let mut events = vec![];
    // for (mut collisionBot, transform) in query.iter_mut() {
    //     let len = collisionBot.collisionInner.other.len();
    //     if len != 0 {
    //         events.push(collisionBot.collisionInner.other.clone());
    //         println!("scopeOther:{:?}", collisionBot.collisionInner.other);
    //         collisionBot.collisionInner.other.clear();
    //     }
    // }

    // for event in events {
    //     for targetId in event {
    //         // other事件的一个东西
    //         let mut target = query.get(targetId).unwrap();
    //         println!(
    //             "发现情况 {:?} {:?}  other:{:?} other: {:?}",
    //             target.1.translation, targetId, target.0.id, target.0.other
    //         );
    //         // 判断是否是自己
    //         // 判断是否是别人
    //     }
    // }
}

// ai设计动作去处理
