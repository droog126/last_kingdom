use crate::instance::utils::create_instance_collision;
use crate::instance::{InstanceCamp, InstanceType};
use crate::state::loading::SpriteCenter;
use crate::systems::collision::{CollisionResultArr, _repel};
use crate::systems::debug::DebugStatus;
use crate::systems::input::InsInput;
use crate::systems::instance::shadow::ShadowAsset;
use crate::systems::stateMachine::{AnimationState, StateChangeEvt, StateInfo, StateMachine};
use bevy::math::Vec3Swizzles;
use bevy_prototype_lyon::prelude::*;

use bevy::prelude::*;

use super::CollisionType;

// res
pub struct GLobalPlayerID(pub Entity);

//component
#[derive(Component)]
pub struct PlayerAnimationTag;

#[derive(Component, Debug)]
// #[reflect(Component)]
pub struct PlayerProps {
    pub spd: f32,
}

#[derive(Component)]
pub struct PlayerTag;

fn getPlayerSprite(animationState: &AnimationState) -> StateInfo {
    match (animationState.0) {
        StateMachine::Idle => StateInfo {
            startIndex: 0,
            endIndex: 0,
            spriteName: "player".to_string(),
        },
        StateMachine::Walk => StateInfo {
            startIndex: 8,
            endIndex: 15,
            spriteName: "player".to_string(),
        },
        _ => StateInfo {
            startIndex: 0,
            endIndex: 0,
            spriteName: "player".to_string(),
        },
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
    mut local: Local<bool>,
    mut commands: Commands,
    mut spriteCenter: ResMut<SpriteCenter>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    shadowHandle: Res<ShadowAsset>,
) {
    println!("我是否是第一次调用{:?}", local);

    if (*local == false) {
        let texture_handle = asset_server.load("sprite/player_sheet.png");
        let sprite_atlas = TextureAtlas::from_grid_with_padding(
            texture_handle.clone(),
            Vec2::new(32.0, 50.0),
            8,
            2,
            Vec2::new(0.0, 0.0),
        );

        let sprite_handle = texture_atlases.add(sprite_atlas);
        spriteCenter.0.insert("player".to_string(), sprite_handle);

        *local = true;
    }

    for _ in 0..1 {
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
        let animationInstanceId = commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 20.0, 10.0),
                    ..Default::default()
                },
                texture_atlas: spriteCenter.0.get("player").unwrap().clone(),
                ..Default::default()
            })
            .insert(AnimationState(StateMachine::Idle, 1.0, getPlayerSprite))
            .insert(Name::new("playerAnimation".to_string()))
            .insert(PlayerAnimationTag)
            .id();

        let shape = shapes::Rectangle {
            extents: Vec2::new(20.0, 10.0),
            origin: RectangleOrigin::Center,
        };

        let instanceId = create_instance_collision(
            &mut commands,
            InstanceType::Player,
            InstanceCamp::Friendly,
            Some(playerCollisionExclude),
            0.0,
            0.0,
            20.0,
            10.0,
        );

        // animation后置添加
        commands.entity(animationInstanceId);

        // 实体后置添加
        commands
            .entity(instanceId)
            .insert(PlayerProps { spd: 200.0 })
            .insert(InsInput {
                ..Default::default()
            })
            .insert(PlayerTag)
            .insert(Name::new("player"))
            .push_children(&[animationInstanceId, shadowId]);

        commands.insert_resource(GLobalPlayerID(instanceId));
    }
}

pub fn player_step(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &PlayerProps,
            &InsInput,
            &mut CollisionResultArr,
            &Children,
        ),
        With<PlayerTag>,
    >,

    mut changeStateEvent: EventWriter<StateChangeEvt>,
    debugStatus: Res<DebugStatus>,
) {
    // 有输入=>移动逻辑
    if debugStatus.camera_debug {
        return;
    }

    let mut nextLen = Vec2::splat(0.0);

    for (mut trans, props, input, mut collisionResultArr, children) in query.iter_mut() {
        let animationInstanceId = children[0];
        if input.dir.length() == 0.0 {
            changeStateEvent.send(StateChangeEvt {
                ins: animationInstanceId,
                newState: StateMachine::Idle,
                xDir: input.dir.x,
            });
        } else {
            changeStateEvent.send(StateChangeEvt {
                ins: animationInstanceId,
                newState: StateMachine::Walk,
                xDir: input.dir.x,
            });
            nextLen.x = input.dir.x * props.spd * time.delta_seconds();
            nextLen.y = input.dir.y * props.spd * time.delta_seconds();
        }

        // println!("看看当前碰撞结果{:?}", collisionResultArr);
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
    }
}
