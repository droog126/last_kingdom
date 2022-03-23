use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::state::loading::SpriteSheetCollection;

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
    pub maxIndex: usize,
    // pub spriteFile: Handle<TextureAtlas>,
}

#[derive(Component, Debug, Clone)]

pub struct InsState(pub StateMachine);

pub trait Info {
    fn _get(&self) -> StateInfo;
}

pub struct StateChangeEvt {
    pub ins: Entity,
    pub newState: StateMachine,
}

pub struct StateMachinePlugin;
impl Plugin for StateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StateChangeEvt>()
            .add_system(state_trigger.label("stateUpdate"))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.015))
                    .with_system(sprite_update),
            );
    }
}
fn state_trigger(
    mut stateChangeRead: EventReader<StateChangeEvt>,
    mut query: Query<(&mut InsState, &mut TextureAtlasSprite)>,
    // spriteSheetCollection: Res<SpriteSheetCollection>,
) {
    for ev in stateChangeRead.iter() {
        if let Ok((mut insState, mut sprite)) = query.get_mut(ev.ins) {
            if (insState.0 != ev.newState) {
                insState.0 = ev.newState;
                sprite.index = 0;
            }

            let spriteName = insState._get();

            // println!("curState:{:?} , curStateInfo:{:?}", insState.0, spriteName);
        }
    }
}

fn sprite_update(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut InsState,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut insState, mut sprite, spriteTexture) in query.iter_mut() {
        let texture_atlas = texture_atlases.get(spriteTexture).unwrap();
        sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
    }
}
