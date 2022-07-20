use crate::state::loading::{ImageCenter, TextureAtlasCenter};
use crate::systems::debug::DebugStatus;
use crate::systems::input::InsInput;
use crate::systems::instance::animation::{AnimationInfo, AnimationMachine, AnimationValue, StateChangeEvt};
use crate::systems::instance::attack::AttackStorehouseArr;
use crate::systems::instance::basicCreate::create_instance_collision;
use crate::systems::instance::collision::{CollisionResultArr, _repel};
use crate::systems::instance::props::{BasicProps, InstanceProps};
use crate::systems::timeLine::{self, TimeLine};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use super::*;

// res
pub struct GLobalPlayerID(pub Entity);

//component
#[derive(Component)]
pub struct PlayerAnimationTag;

#[derive(Component)]
pub struct PlayerTag;

fn getPlayerSprite(animationValue: &AnimationValue) -> AnimationInfo {
    match *animationValue {
        AnimationValue::Idle => AnimationInfo { startIndex: 0, endIndex: 0, spriteName: "player".to_string() },
        AnimationValue::Walk => AnimationInfo { startIndex: 8, endIndex: 15, spriteName: "player".to_string() },
        _ => AnimationInfo { startIndex: 0, endIndex: 0, spriteName: "player".to_string() },
    }
}

fn playerCollisionExclude(
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

pub fn player_create(
    mut commands: &mut Commands,
    textureAtlasCenter: &Res<TextureAtlasCenter>,
    imageCenter: &Res<ImageCenter>,
    x: f32,
    y: f32,
) -> Entity {
    // 阴影实体
    let shadowId = commands
        .spawn_bundle(SpriteBundle {
            texture: imageCenter.0.get("shadow").unwrap().clone(),
            transform: Transform { scale: Vec3::new(1.0, 0.5, 0.0), ..default() },
            ..default()
        })
        .id();

    //  动画实体
    let animationId = commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform { translation: Vec3::new(0.0, 20.0, 10.0), ..Default::default() },
            texture_atlas: textureAtlasCenter.0.get("player").unwrap().clone(),
            ..Default::default()
        })
        .insert(AnimationMachine { value: AnimationValue::Idle, progress: 0.0, config: getPlayerSprite })
        .insert(Name::new("playerAnimation".to_string()))
        .insert(PlayerAnimationTag)
        .id();

    // 人物实体
    let instanceId = create_instance_collision(
        &mut commands,
        InstanceType::Player,
        InstanceCamp::Friendly,
        Some(playerCollisionExclude),
        x,
        y,
        10.0,
        10.0,
        InstanceProps::new(BasicProps {
            hp: 20.,
            energy: 20.,
            speed: 200.,
            bouncing: 400.,
            maxHp: 20.,
            maxEnergy: 20.,
            maxSpeed: 200.,
            maxBouncing: 400.,
        }),
    );

    commands
        .entity(instanceId)
        .insert(InsInput { ..Default::default() })
        .insert(PlayerTag)
        .insert(Name::new("player"));

    // 添加children
    commands.entity(instanceId).push_children(&[animationId, shadowId]);

    commands.insert_resource(GLobalPlayerID(instanceId));

    instanceId
}

pub fn player_step(
    time: Res<Time>,
    timeLine: Res<TimeLine>,
    mut query: Query<
        (
            &mut Transform,
            &mut InstanceProps,
            &InsInput,
            &mut CollisionResultArr,
            &mut AttackStorehouseArr,
            &Children,
        ),
        With<PlayerTag>,
    >,

    mut changeStateEvent: EventWriter<StateChangeEvt>,
    debugStatus: Res<DebugStatus>,
) {
    let timeLineRaw = timeLine.0;
    // 有输入=>移动逻辑
    if debugStatus.camera_debug {
        return;
    }

    let mut nextLen = Vec2::splat(0.0);

    for (mut trans, mut instanceProps, input, mut collisionResultArr, mut attackStorehouseArr, children) in
        query.iter_mut()
    {
        let props = instanceProps.get();
        let animationInstanceId = children[0];
        if input.dir.length() == 0.0 {
            changeStateEvent.send(StateChangeEvt {
                ins: animationInstanceId,
                newValue: AnimationValue::Idle,
                xDir: input.dir.x,
            });
        } else {
            changeStateEvent.send(StateChangeEvt {
                ins: animationInstanceId,
                newValue: AnimationValue::Walk,
                xDir: input.dir.x,
            });
            nextLen.x = input.dir.x * props.speed * time.delta_seconds();
            nextLen.y = input.dir.y * props.speed * time.delta_seconds();
        }

        // println!("看看当前碰撞结果{:?}", collisionResultArr);
        for collisionItem in collisionResultArr.arr.iter() {
            nextLen += _repel(&trans.translation.xy(), &collisionItem.shape.pos, None, None)
        }
        collisionResultArr.arr.clear();

        trans.translation.x += nextLen.x;
        trans.translation.y += nextLen.y;

        // 处理攻击事件仓库

        attackStorehouseArr.arr.retain_mut(|e| timeLineRaw < e.nextTime);
        for attackEvent in attackStorehouseArr.arr.iter_mut() {
            instanceProps.sub_hp(attackEvent.damage);
            attackEvent.damage = 0.0;
            if let Some(repelData) = attackEvent.repelData.as_mut() {
                trans.translation += (repelData.dif * time.delta_seconds());
            }
        }
        // attackStorehouseArr.arr.clear();
    }
}
