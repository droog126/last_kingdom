use crate::state::loading::PlayerSheet;
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
            StateMachine::Idle => StateInfo { maxIndex: 2 },
            StateMachine::Walk => StateInfo { maxIndex: 8 },
            _ => StateInfo { maxIndex: 1 },
        }
    }
}

pub fn player_create(mut commands: Commands, playerSheet: Res<PlayerSheet>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            texture_atlas: playerSheet.idle.clone(),
            ..Default::default()
        })
        .insert(PlayerProps { spd: 4.0 })
        .insert(InsInput {
            ..Default::default()
        })
        .insert(InsState {
            ..Default::default()
        })
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
