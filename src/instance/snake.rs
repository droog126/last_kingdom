use crate::state::loading::SpriteCenter;
use crate::systems::collision::{CollisionBot, CollisionID};
use crate::systems::debug::DebugStatus;
use crate::systems::instance::shadow::ShadowAsset;
use crate::systems::instance::InstanceCollisionTag;
use crate::systems::stateMachine::{
    AnimationInstanceId, InsState, StateChangeEvt, StateInfo, StateMachine,
};
use crate::systems::timeLine::TimeLine;
use crate::utils::random::{random_Vec2, random_in_unlimited, random_range};
use bevy::utils::HashMap;

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

#[derive(Component, Debug)]
pub struct SnakeAi {
    target: Option<Entity>,
    state: AiState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AiState {
    stroll { dir: Vec2, nextTime: i32 },
    daze,
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
    mut spriteCenter: &mut ResMut<SpriteCenter>,
    shadowHandle: &mut ResMut<ShadowAsset>,
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
        .insert(InsState(StateMachine::Idle, 1.0, getSnakeSprite))
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
        .insert(SnakeAi {
            target: None,
            state: AiState::daze,
        })
        .insert(AnimationInstanceId(instanceId))
        .push_children(&[instanceId, shadowId, scopeCollisionId]);

    commands
        .entity(scopeCollisionId)
        .insert(SnakeScopeCollisionTag);
}

// 运行限制条件，snake确实存在  可能需要一张表来维护
pub fn snake_step(
    time: Res<Time>,
    mut changeStateEvent: EventWriter<StateChangeEvt>,
    debugStatus: Res<DebugStatus>,
    mut set: ParamSet<(
        Query<&mut CollisionBot, With<SnakeScopeCollisionTag>>,
        Query<(&Transform, &InstanceCategory), With<InstanceCollisionTag>>,
        Query<(&mut Transform, &mut SnakeAi, &AnimationInstanceId), With<SnakeCollisionTag>>,
    )>,
    timeLine: Res<TimeLine>,
) {
    let timeLineRaw = timeLine.0;
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
    return;
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
        if let Ok((mut instanceTransform, mut snakeAi, animationInstanceId)) = query.get_mut(entity)
        {
            let len = otherInfos.len();

            if len >= 1 {
                // 遇到目标
                let (targetTransform, _) = &otherInfos[0];
                let mut diff = targetTransform.translation - instanceTransform.translation;
                diff = diff.normalize();
                instanceTransform.translation += diff * time.delta_seconds() * 100.0;

                let newActState = StateMachine::Walk;
                let newActXScale = diff.x;
                changeStateEvent.send(StateChangeEvt {
                    ins: animationInstanceId.0,
                    newState: newActState,
                    xDir: newActXScale,
                });
            } else {
                // 没有遇到目标
                match snakeAi.state {
                    AiState::stroll { dir, nextTime } => {
                        // println!("stroll {:?} {:?}", dir, nextTime);
                        instanceTransform.translation +=
                            dir.extend(0.0) * time.delta_seconds() * 100.0;

                        if timeLineRaw > nextTime {
                            snakeAi.state = AiState::daze;
                        }
                        let newActState = StateMachine::Walk;
                        let newActXScale = dir.x;
                        changeStateEvent.send(StateChangeEvt {
                            ins: animationInstanceId.0,
                            newState: newActState,
                            xDir: newActXScale,
                        });
                    }
                    AiState::daze => {
                        if random_in_unlimited(1.0 / 10.0, time.delta_seconds()) {
                            snakeAi.state = AiState::stroll {
                                dir: random_Vec2(),
                                nextTime: timeLineRaw
                                    + (60.0 * random_range::<f32>(0.5, 2.0)) as i32,
                            };
                        }

                        let newActState = StateMachine::Idle;
                        changeStateEvent.send(StateChangeEvt {
                            ins: animationInstanceId.0,
                            newState: newActState,
                            xDir: 0.0,
                        });
                    }
                }
            }
        }
    }
}
