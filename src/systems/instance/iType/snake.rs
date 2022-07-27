use crate::state::loading::{ImageCenter, TextureAtlasCenter};
use crate::systems::debug::DebugStatus;
use crate::systems::instance::animation::{AnimationInfo, AnimationMachine, AnimationValue, StateChangeEvt};
use crate::systems::instance::attack::{create_attack_box, AttackEventPart, AttackStorehouseArr, RepelData};
use crate::systems::instance::basicCreate::{create_dyn_collision, create_scope_collision};
use crate::systems::instance::collision::{CollisionResultArr, _repel};
use crate::systems::instance::props::{BasicProps, InstanceProps};
use crate::systems::timeLine::TimeLine;
use crate::utils::random::{random_Vec2, random_in_unlimited, random_range};
use bevy::math::Vec3Swizzles;

use bevy::prelude::*;

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
pub struct SnakeAi {
    target: Option<Entity>,
    state: AiState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AiState {
    Stroll { dir: Vec2, nextTime: i32 },
    Daze,
    Chase,
    Attack { pos: Vec2, happenTime: i32 },
}

// 配置
pub fn getSnakeSprite(animationValue: &AnimationValue) -> AnimationInfo {
    match *animationValue {
        AnimationValue::Idle => AnimationInfo { startIndex: 0, endIndex: 7, spriteName: "snake".to_string() },
        AnimationValue::Walk => AnimationInfo { startIndex: 8, endIndex: 15, spriteName: "snake".to_string() },
        AnimationValue::Attack => AnimationInfo { startIndex: 16, endIndex: 21, spriteName: "snake".to_string() },
        _ => AnimationInfo { startIndex: 0, endIndex: 0, spriteName: "snake".to_string() },
    }
}
pub fn snakeCollisionExclude(
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

pub fn snakeScopeCollisionExclude(
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

// create
pub fn snake_create(
    mut commands: &mut Commands,
    textureAtlasCenter: &Res<TextureAtlasCenter>,
    imageCenter: &Res<ImageCenter>,
    x: f32,
    y: f32,
) {
    // 阴影实体
    let shadowId = commands
        .spawn_bundle(SpriteBundle {
            texture: imageCenter.0.get("shadow").unwrap().clone(),
            transform: Transform { scale: Vec3::new(1.0, 0.5, 0.0), ..default() },
            ..default()
        })
        .id();

    // 动画实体
    let animationId = commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform { translation: Vec3::new(4.0, 14.0, 10.0), ..Default::default() },
            texture_atlas: textureAtlasCenter.0.get("snake").unwrap().clone(),
            ..Default::default()
        })
        .insert(AnimationMachine { value: AnimationValue::Idle, config: getSnakeSprite, progress: 0.0 })
        .insert(Name::new("snake".to_string()))
        .insert(SnakeAnimationTag)
        .id();

    // 人物实体
    let instanceId = create_dyn_collision(
        &mut commands,
        x,
        y,
        20.0,
        10.0,
        InstanceType::Snake,
        InstanceCamp::Hostile,
        Some(snakeCollisionExclude),
    );
    commands
        .entity(instanceId)
        .insert(Name::new("snake"))
        .insert(SnakeTag)
        .insert(InstanceProps::new(BasicProps {
            hp: 20.,
            energy: 20.,
            speed: 300.,
            bouncing: 400.,
            maxHp: 20.,
            maxEnergy: 20.,
            maxSpeed: 300.,
            maxBouncing: 400.,
        }))
        .insert(SnakeAi { target: None, state: AiState::Daze });

    // 侦查盒子
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

    commands.entity(scopeCollisionId).insert(SnakeScopeTag);

    // 添加children
    commands.entity(instanceId).push_children(&[animationId, shadowId, scopeCollisionId]);
}

// 运行限制条件，snake确实存在  可能需要一张表来维护
pub fn snake_step(
    mut commands: Commands,
    textureAtlasCenter: Res<TextureAtlasCenter>,
    imageCenter: Res<ImageCenter>,
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
            &mut AttackStorehouseArr,
            &mut InstanceProps,
        ),
        (With<SnakeTag>, Without<SnakeScopeTag>),
    >,
    mut scopeQuery: Query<(&mut CollisionResultArr), (With<SnakeScopeTag>, Without<SnakeTag>)>,
    mut animationQuery: Query<&mut AnimationMachine, With<SnakeAnimationTag>>,
) {
    let timeLineRaw = timeLine.0;

    // let mut instanceQuery = set.p0();
    for (
        entity,
        mut trans,
        mut collisionResultArr,
        mut snakeAi,
        children,
        mut attackStorehouseArr,
        mut instanceProps,
    ) in instanceQuery.iter_mut()
    {
        let scopeEntityId = children[2];

        // feat:相互碰撞
        let mut nextLen = Vec2::splat(0.0);
        let animationId = children[0];
        let scopeCollisionId = children[2];

        for collisionItem in collisionResultArr.arr.iter() {
            nextLen += _repel(&trans.translation.xy(), &collisionItem.shape.pos, None, None)
        }
        collisionResultArr.arr.clear();

        // hitBox碰撞事件
        attackStorehouseArr.arr.retain_mut(|e| timeLineRaw < e.nextTime);
        for attackEvent in attackStorehouseArr.arr.iter_mut() {
            instanceProps.sub_hp(attackEvent.damage);
            attackEvent.damage = 0.0;
            println!("蛇被打到了!!,{:?}", instanceProps.get());
            if let Some(repelData) = attackEvent.repelData.as_mut() {
                trans.translation += (repelData.dif * time.delta_seconds());
            }
        }

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
                    if (diff.xy().length() < 20.0) {
                        snakeAi.state = AiState::Attack { pos: target.shape.pos, happenTime: timeLineRaw };
                    }
                    diff = diff.normalize();
                    trans.translation += diff * time.delta_seconds() * 100.0;
                    changeStateEvent.send(StateChangeEvt {
                        ins: animationId,
                        newValue: AnimationValue::Walk,
                        xDir: diff.x,
                    });
                }
                AiState::Attack { pos, happenTime } => {
                    let mut _progress = 0.0;
                    if let Ok(mut animationMachine) = animationQuery.get_mut(children[0]) {
                        _progress = animationMachine.progress;
                        animationMachine.progress = 0.0;
                    }

                    if _progress == 1.0 {
                        snakeAi.state = AiState::Daze;
                        create_attack_box(
                            &mut commands,
                            imageCenter.0.get("circle").unwrap().clone(),
                            InstanceType::Snake,
                            InstanceCamp::Hostile,
                            Some(|instanceType, collisionType, instanceCamp| {
                                if (instanceType == &InstanceType::Player) {
                                    false
                                } else {
                                    true
                                }
                            }),
                            AttackEventPart {
                                damage: 2.0,
                                nextTime: timeLineRaw + 20,
                                repelData: Some(RepelData { dif: diff.normalize() * 60.0, timeLen: 20 }),
                            },
                            pos.x,
                            pos.y,
                            20.,
                            20.,
                        );
                    }
                    changeStateEvent.send(StateChangeEvt {
                        ins: animationId,
                        newValue: AnimationValue::Attack,
                        xDir: diff.x,
                    })
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
                        newValue: AnimationValue::Walk,
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
                        newValue: AnimationValue::Idle,
                        xDir: 0.0,
                    });
                }
                AiState::Chase => {
                    snakeAi.state = AiState::Daze;
                }
                AiState::Attack { .. } => {
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
