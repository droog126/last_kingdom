use crate::state::loading::SpriteCenter;
use crate::systems::collision::{CollisionID, CollisionTag};
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

    if (*local == true) {
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

        *local = false;
    }

    for _ in 0..1 {
        let shape = shapes::Rectangle {
            extents: Vec2::new(20.0, 10.0),
            origin: RectangleOrigin::Center,
        };
        let collisionChildId = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::CYAN),
                    outline_mode: StrokeMode::new(Color::BLACK, 1.0),
                },
                Transform::from_translation(Vec3::new(0., -20.0, 0.0)),
            ))
            .insert(CollisionTag)
            .insert(Name::new("collision"))
            .insert(Visibility { is_visible: false })
            .id();

        let parentId = commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 10.0),
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
            .insert(CollisionID(collisionChildId))
            .push_children(&[collisionChildId])
            .id();

        commands.insert_resource(GLobalPlayerID(parentId));
    }
}

pub fn player_step(
    time: Res<Time>,
    mut player_query: Query<
        (
            Entity,
            &mut Transform,
            &PlayerProps,
            &mut InsInput,
            &mut InsState,
        ),
        With<PlayerTag>,
    >,
    mut changeStateSend: EventWriter<StateChangeEvt>,
    debugStatus: Res<DebugStatus>,
) {
    if (debugStatus.camera_debug) {
        return;
    }
    for (entity, mut trans, props, mut input, mut insState) in player_query.iter_mut() {
        if (input.dir.length() == 0.0) {
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
            trans.translation.x += input.dir.x * props.spd * time.delta_seconds();
            trans.translation.y += input.dir.y * props.spd * time.delta_seconds();
        }
    }
}
