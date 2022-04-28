use crate::state::loading::SpriteCenter;
use crate::systems::collision::{CollisionBot, CollisionConfig, CollisionDynTag, CollisionID};
use crate::systems::debug::DebugStatus;
use crate::systems::input::InsInput;
use crate::systems::stateMachine::{Info, InsState, StateChangeEvt, StateInfo, StateMachine};
use bevy_prototype_lyon::prelude::*;

use bevy::prelude::*;

// res
pub struct GLobalPlayerID(pub Entity);

//component
#[derive(Component)]
pub struct PlayerTag;

#[derive(Component, Debug)]
// #[reflect(Component)]
pub struct PlayerProps {
    pub spd: f32,
}

#[derive(Component)]
pub struct PlayerCollisionDynTag;

impl Info for InsState {
    fn _get(&self) -> StateInfo {
        match (self.0) {
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
}

pub fn player_create(
    mut local: Local<bool>,
    mut commands: Commands,
    mut spriteCenter: ResMut<SpriteCenter>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
        let instanceId = commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 20.0, 10.0),
                    ..Default::default()
                },
                texture_atlas: spriteCenter.0.get("player").unwrap().clone(),
                ..Default::default()
            })
            .insert(PlayerProps { spd: 300.0 })
            .insert(InsInput {
                ..Default::default()
            })
            .insert(InsState(StateMachine::Idle))
            .insert(Name::new("player".to_string()))
            .insert(PlayerTag)
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
                Transform::from_translation(Vec3::new(0., 0.0, 0.0)),
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
            .insert(PlayerCollisionDynTag)
            .insert(Name::new("playerCollision"))
            .insert(Visibility { is_visible: false })
            .push_children(&[instanceId])
            .id();

        // player后置添加
        commands.entity(instanceId).insert(CollisionID(collisionId));

        commands.insert_resource(GLobalPlayerID(instanceId));
    }
}

pub fn player_step(
    time: Res<Time>,
    mut set: ParamSet<(
        Query<
            (
                Entity,
                &mut Transform,
                &PlayerProps,
                &InsInput,
                &mut InsState,
            ),
            With<PlayerTag>,
        >,
        Query<(&mut Transform), With<PlayerCollisionDynTag>>,
    )>,

    mut changeStateSend: EventWriter<StateChangeEvt>,
    debugStatus: Res<DebugStatus>,
) {
    // 有输入=>移动逻辑
    if debugStatus.camera_debug {
        return;
    }
    let mut playerQuery = set.p0();

    let mut nextLen = Vec2::splat(0.0);

    for (entity, mut trans, props, input, mut insState) in playerQuery.iter_mut() {
        if input.dir.length() == 0.0 {
            changeStateSend.send(StateChangeEvt {
                ins: entity,
                newState: StateMachine::Idle,
                xDir: input.dir.x,
            });
        } else {
            changeStateSend.send(StateChangeEvt {
                ins: entity,
                newState: StateMachine::Walk,
                xDir: input.dir.x,
            });
            nextLen.x = input.dir.x * props.spd * time.delta_seconds();
            nextLen.y = input.dir.y * props.spd * time.delta_seconds();
        }
    }

    let mut collisionQuery = set.p1();

    for mut transform in collisionQuery.iter_mut() {
        transform.translation.x += nextLen.x;
        transform.translation.y += nextLen.y;
    }
}
