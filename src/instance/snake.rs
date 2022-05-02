use crate::state::loading::SpriteCenter;
use crate::systems::collision::{CollisionBot, CollisionConfig, CollisionDynTag, CollisionID};
use crate::systems::debug::DebugStatus;
use crate::systems::instance::shadow::ShadowAsset;
use crate::systems::stateMachine::{InsState, StateChangeEvt, StateInfo, StateMachine};
use bevy_prototype_lyon::prelude::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct SnakeTag;

#[derive(Component, Debug)]
pub struct SnakeProps {
    pub spd: f32,
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

    let shape = shapes::Rectangle {
        extents: Vec2::new(20.0, 10.0),
        origin: RectangleOrigin::Center,
    };
    let collisionId = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::from_translation(Vec3::new(x, y, 1.0)),
        ))
        .insert(CollisionDynTag)
        .insert(CollisionBot {
            pos: Vec2::new(0.0, 0.0),
            force: Vec2::new(0.0, 0.0),
            wall_move: [None; 2],
        })
        .insert(CollisionConfig {
            width: 20,
            height: 10,
        })
        .insert(Name::new("snakeCollision"))
        .insert(Visibility { is_visible: false })
        .push_children(&[instanceId, shadowId])
        .id();

    // player后置添加
    commands.entity(instanceId).insert(CollisionID(collisionId));
}

pub fn snake_step(
    time: Res<Time>,
    mut changeStateSend: EventWriter<StateChangeEvt>,
    debugStatus: Res<DebugStatus>,
) {
}
