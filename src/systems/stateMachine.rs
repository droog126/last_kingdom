use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::state::loading::{SpriteCenter};

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
#[reflect(Component)]
pub enum StateMachine {
    Idle,
    Walk,
}
impl Default for StateMachine {
    fn default() -> Self {
        StateMachine::Idle
    }
}

// 和struct相关的函数闭包需求就挂在这个struct的trait上

#[derive(Debug)]
pub struct StateInfo {
    pub startIndex: usize,
    pub endIndex: usize,
    pub spriteName: String,
}

#[derive(Component, Debug, Clone)]

pub struct InsState(pub StateMachine);

pub trait Info {
    fn _get(&self) -> StateInfo;
}

pub struct StateChangeEvt {
    pub ins: Entity,
    pub newState: StateMachine,
    pub xDir: f32,
}

pub struct StateMachinePlugin;
impl Plugin for StateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StateChangeEvt>()
            .add_system(state_trigger.label("stateUpdate"))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.1))
                    .with_system(sprite_update),
            );
    }
}
fn state_trigger(
    mut stateChangeRead: EventReader<StateChangeEvt>,
    mut query: Query<(
        &mut InsState,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    mut spriteCenter: ResMut<SpriteCenter>,
) {
    for ev in stateChangeRead.iter() {
        if let Ok((mut insState, mut sprite, mut sprite_handle)) = query.get_mut(ev.ins) {
            if (insState.0 != ev.newState) {
                insState.0 = ev.newState;
                sprite.index = 0;

                let StateInfo {
                    spriteName,
                    startIndex,
                    endIndex,
                } = insState._get();

                let newSpriteHandle = spriteCenter.0.get(&spriteName).unwrap();

                *sprite_handle = newSpriteHandle.clone();
                sprite.index = startIndex;
            }

            if (ev.xDir > 0.0) {
                sprite.flip_x = false;
            }
            if (ev.xDir < 0.0) {
                sprite.flip_x = true;
            }
        }
    }
}

fn sprite_update(mut query: Query<(&mut InsState, &mut TextureAtlasSprite)>) {
    for (mut insState, mut sprite) in query.iter_mut() {
        let StateInfo {
            startIndex,
            endIndex,
            spriteName,
        } = insState._get();

        if (sprite.index >= endIndex) {
            sprite.index = startIndex;
        } else {
            sprite.index += 1;
        }
    }
}
