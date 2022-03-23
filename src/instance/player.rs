use crate::state::loading::{SpriteCenter, SpriteSheetCollection};
use crate::systems::input::InsInput;
use crate::systems::stateMachine::{Info, InsState, StateChangeEvt, StateInfo, StateMachine};

use bevy::prelude::*;

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
            // StateMachine::Idle => StateInfo {
            //     maxIndex: 1,
            //     spriteFile: spriteSheetCollection.p,
            // },
            // StateMachine::Walk => StateInfo {
            //     maxIndex: 8,
            //     spriteFile: "player_walk".to_string(),
            // },
            _ => StateInfo {
                maxIndex: 1,
                // spriteFile: self.1.player_idle.clone(),
            },
        }
    }
}

pub fn player_create(
    mut local: Local<bool>,
    mut commands: Commands,
    mut spriteSheetCollection: ResMut<SpriteSheetCollection>,
    mut spriteCenter: ResMut<SpriteCenter>,
) {
    println!("我是否是第一次调用{:?}", local);

    if (*local == true) {
        spriteCenter.0.insert(
            "playerIdle".to_string(),
            spriteSheetCollection.player_idle.clone(),
        );
        *local = false;
    }
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            texture_atlas: spriteSheetCollection.player_idle.clone(),
            ..Default::default()
        })
        .insert(PlayerProps { spd: 4.0 })
        .insert(InsInput {
            ..Default::default()
        })
        .insert(InsState(StateMachine::Idle))
        .insert(PlayerTag);
}

pub fn player_step(
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
) {
    for (entity, mut trans, props, mut input, mut insState) in player_query.iter_mut() {
        if (input.dir.length() == 0.0) {
            changeStateSend.send(StateChangeEvt {
                ins: entity,
                newState: StateMachine::Idle,
            });
        } else {
            changeStateSend.send(StateChangeEvt {
                ins: entity,
                newState: StateMachine::Walk,
            });
            let factor = props.spd;
            trans.translation.x += input.dir.x * factor;
            trans.translation.y += input.dir.y * factor;
        }
    }
}

pub fn player_draw() {}
