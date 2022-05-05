use crate::state::loading::SpriteCenter;
use crate::systems::collision::{CollisionBot, CollisionID};
use crate::systems::debug::DebugStatus;
use crate::systems::instance::shadow::ShadowAsset;
use crate::systems::instance::InstanceCollisionTag;
use crate::systems::stateMachine::{InsState, StateChangeEvt, StateInfo, StateMachine};
use bevy_prototype_lyon::prelude::*;

use bevy::prelude::*;

use super::utils::{create_instance_collision, create_scope_collision};

#[derive(Component)]
pub struct SnakeTag;

#[derive(Component, Debug)]
pub struct SnakeProps {
    pub spd: f32,
}
#[derive(Component)]
pub struct SnakeCollisionTag;

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
        .push_children(&[instanceId, shadowId, scopeCollisionId]);
}

// 运行限制条件，snake确实存在  可能需要一张表来维护
pub fn snake_step(
    time: Res<Time>,
    mut changeStateSend: EventWriter<StateChangeEvt>,
    debugStatus: Res<DebugStatus>,
    mut set: ParamSet<(
        Query<(&mut Transform, &Children), With<SnakeCollisionTag>>,
        Query<(&Transform, With<CollisionBot>)>,
    )>,
) {
    let mut query = set.p0();
    for (mut transform, children) in query.iter_mut() {
        // println!("children: {:?}", children);
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
