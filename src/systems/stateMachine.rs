use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use crate::state::loading::SpriteCenter;

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

#[derive(Component)]
pub struct AnimationInstanceId(pub Entity);

#[derive(Debug)]
pub struct StateInfo {
    pub startIndex: usize,
    pub endIndex: usize,
    pub spriteName: String,
}

#[derive(Component, Clone)]

pub struct AnimationState(
    pub StateMachine,
    pub f32,
    pub fn(&AnimationState) -> StateInfo,
);

impl AnimationState {
    fn get(&self) -> StateInfo {
        (self.2)(self)
    }
}

pub struct StateChangeEvt {
    pub ins: Entity,
    pub newState: StateMachine,
    pub xDir: f32,
}

pub struct NextActMap(pub HashMap<Entity, NextActMapValue>);
pub struct NextActMapValue {
    pub nextState: StateMachine,
    pub nextXScale: f32,
}

pub struct StateMachinePlugin;
impl Plugin for StateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NextActMap(HashMap::new()));

        app.add_event::<StateChangeEvt>()
            .add_system(state_trigger.label("stateUpdate"))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.1))
                    .with_system(sprite_update),
            );
        // .add_system(step);
    }
}

fn state_trigger(
    mut stateChangeRead: EventReader<StateChangeEvt>,
    mut query: Query<(
        &mut AnimationState,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &mut Transform,
    )>,
    mut spriteCenter: ResMut<SpriteCenter>,
) {
    for ev in stateChangeRead.iter() {
        if let Ok((mut animationState, mut sprite, mut sprite_handle, mut transform)) =
            query.get_mut(ev.ins)
        {
            if (animationState.0 != ev.newState) {
                animationState.0 = ev.newState;
                sprite.index = 0;

                let StateInfo {
                    spriteName,
                    startIndex,
                    endIndex,
                } = animationState.get();

                let newSpriteHandle = spriteCenter.0.get(&spriteName).unwrap();

                *sprite_handle = newSpriteHandle.clone();
                sprite.index = startIndex;
            }

            if (ev.xDir > 0.0) {
                sprite.flip_x = false;
                transform.translation.x = transform.translation.x.abs();
            }
            if (ev.xDir < 0.0) {
                sprite.flip_x = true;
                transform.translation.x = -transform.translation.x.abs();
            }
        }
    }
}

fn sprite_update(mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite)>) {
    for (mut animationState, mut sprite) in query.iter_mut() {
        let StateInfo {
            startIndex,
            endIndex,
            spriteName,
        } = animationState.get();

        if (sprite.index >= endIndex) {
            sprite.index = startIndex;
        } else {
            sprite.index += 1;
        }
    }
}
