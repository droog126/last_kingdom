use crate::state::loading::SpriteCenter;
use crate::systems::collision::{CollisionResultArr, _repel};
use crate::systems::debug::DebugStatus;
use crate::systems::instance::shadow::ShadowAsset;
use crate::systems::stateMachine::{AnimationState, StateChangeEvt, StateInfo, StateMachine};
use crate::systems::timeLine::TimeLine;
use crate::utils::random::{random_Vec2, random_in_unlimited, random_range};
use bevy::math::Vec3Swizzles;
use bevy::utils::HashMap;

use bevy::prelude::*;

use super::utils::{create_attack_box, create_instance_collision, create_scope_collision};
use super::{CollisionType, InstanceCamp, InstanceType};
#[derive(Component)]
pub struct SnakeTag;
#[derive(Component)]
pub struct SnakeAnimationTag;
#[derive(Component, Debug)]
pub struct SnakeScopeTag;
#[derive(Component, Debug)]

pub struct SnakeAttackBoxTag;

#[derive(Component, Debug)]
pub struct SnakeProps {
    pub spd: f32,
}

#[derive(Component, Debug)]
pub struct SnakeAi {
    target: Option<Entity>,
    state: AiState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AiState {
    Stroll { dir: Vec2, nextTime: i32 },
    Daze,
    Chase,
    Attack { pos: Vec2 },
}

fn getSnakeSprite(animationState: &AnimationState) -> StateInfo {
    match (animationState.0) {
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
fn snakeCollisionExclude(
    instanceType: &InstanceType,
    collisionType: &CollisionType,
    instanceCamp: &InstanceCamp,
) -> bool {
    if (collisionType == &CollisionType::Instance) {
        false
    } else {
        true
    }
}

fn snakeScopeCollisionExclude(
    instanceType: &InstanceType,
    collisionType: &CollisionType,
    instanceCamp: &InstanceCamp,
) -> bool {
    if (instanceType == &InstanceType::Player) {
        false
    } else {
        true
    }
}

pub fn snake_create_raw(
    mut commands: &mut Commands,
    spriteCenter: Res<SpriteCenter>,
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
    let animationId = commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(4.0, 14.0, 10.0),
                ..Default::default()
            },
            texture_atlas: spriteCenter.0.get("snake").unwrap().clone(),
            ..Default::default()
        })
        .insert(SnakeProps { spd: 200.0 })
        .insert(AnimationState(StateMachine::Idle, 1.0, getSnakeSprite))
        .insert(Name::new("snake".to_string()))
        .insert(SnakeAnimationTag)
        .id();

    let instanceId = create_instance_collision(
        &mut commands,
        InstanceType::Snake,
        InstanceCamp::Hostile,
        Some(snakeCollisionExclude),
        x,
        y,
        20.0,
        10.0,
    );

    let scopeCollisionId = create_scope_collision(
        &mut commands,
        instanceId,
        InstanceType::Snake,
        InstanceCamp::Hostile,
        Some(snakeScopeCollisionExclude),
        0.0,
        0.0,
        100.0,
        100.0,
    );

    // instance后置添加
    commands
        .entity(instanceId)
        .insert(Name::new("snake"))
        .insert(SnakeTag)
        .insert(SnakeAi {
            target: None,
            state: AiState::Daze,
        })
        .push_children(&[animationId, shadowId, scopeCollisionId]);

    commands.entity(scopeCollisionId).insert(SnakeScopeTag);
}

// 运行限制条件，snake确实存在  可能需要一张表来维护
pub fn snake_step(
    mut commands: Commands,
    spriteCenter: Res<SpriteCenter>,
    shadowHandle: Res<ShadowAsset>,

    time: Res<Time>,
    timeLine: Res<TimeLine>,
    mut changeStateEvent: EventWriter<StateChangeEvt>,
    debugStatus: Res<DebugStatus>,

    mut instanceQuery: Query<
        (
            Entity,
            &mut Transform,
            &mut CollisionResultArr,
            &mut SnakeAi,
            &Children,
        ),
        (With<SnakeTag>, Without<SnakeScopeTag>),
    >,
    mut scopeQuery: Query<&mut CollisionResultArr, (With<SnakeScopeTag>, Without<SnakeTag>)>,
) {
    let timeLineRaw = timeLine.0;

    // let mut instanceQuery = set.p0();
    for (entity, mut trans, mut collisionResultArr, mut snakeAi, children) in
        instanceQuery.iter_mut()
    {
        let scopeEntityId = children[2];

        // feat:相互碰撞
        let mut nextLen = Vec2::splat(0.0);
        let animationId = children[0];
        let scopeCollisionId = children[2];

        for collisionItem in collisionResultArr.arr.iter() {
            nextLen += _repel(
                &trans.translation.xy(),
                &collisionItem.shape.pos,
                None,
                None,
            )
        }
        collisionResultArr.arr.clear();
        trans.translation.x += nextLen.x;
        trans.translation.y += nextLen.y;

        // feat:Ai
        let mut scopeCollisionResultArr = scopeQuery.get_mut(scopeEntityId).unwrap();
        let mut len = scopeCollisionResultArr.arr.len();
        if len > 0 {
            let target = &mut scopeCollisionResultArr.arr[0];
            let mut diff = target.shape.pos.extend(0.0) - trans.translation;

            match snakeAi.state {
                AiState::Stroll { .. } => {
                    snakeAi.state = AiState::Chase;
                }
                AiState::Daze => {
                    snakeAi.state = AiState::Chase;
                }
                AiState::Chase => {
                    if (diff.xy().length() < 30.0) {
                        snakeAi.state = AiState::Attack {
                            pos: target.shape.pos,
                        };
                    }
                    diff = diff.normalize();
                    trans.translation += diff * time.delta_seconds() * 100.0;
                    changeStateEvent.send(StateChangeEvt {
                        ins: animationId,
                        newState: StateMachine::Walk,
                        xDir: diff.x,
                    });
                }
                AiState::Attack { pos } => {
                    create_attack_box(
                        shadowHandle.clone(),
                        spriteCenter.0.get("snake").unwrap().clone(),
                        getSnakeSprite,
                        &mut commands,
                        "_snake",
                        SnakeTag,
                        InstanceType::Snake,
                        InstanceCamp::Hostile,
                        None,
                        pos.x,
                        pos.y,
                        20.,
                        20.,
                    );
                }
            }
        } else {
            match snakeAi.state {
                AiState::Stroll { dir, nextTime } => {
                    trans.translation += dir.extend(0.0) * time.delta_seconds() * 100.0;

                    if timeLineRaw > nextTime {
                        snakeAi.state = AiState::Daze;
                    }

                    changeStateEvent.send(StateChangeEvt {
                        ins: animationId,
                        newState: StateMachine::Walk,
                        xDir: dir.x,
                    });
                }
                AiState::Daze => {
                    if random_in_unlimited(1.0 / 10.0, time.delta_seconds()) {
                        snakeAi.state = AiState::Stroll {
                            dir: random_Vec2(),
                            nextTime: timeLineRaw + (60.0 * random_range::<f32>(0.5, 2.0)) as i32,
                        };
                    }
                    changeStateEvent.send(StateChangeEvt {
                        ins: animationId,
                        newState: StateMachine::Idle,
                        xDir: 0.0,
                    });
                }
                AiState::Chase => {
                    snakeAi.state = AiState::Daze;
                }
                AiState::Attack { pos } => {
                    snakeAi.state = AiState::Daze;
                }
            }
        }
        scopeCollisionResultArr.arr.clear();
    }
    // 内存泄漏
    // for (mut collisionResultArr) in scopeQuery.iter_mut() {
    //     // 按道理来说这个捕获的都给snake了需要验证下
    //     collisionResultArr.arr.clear();
    // }
}
