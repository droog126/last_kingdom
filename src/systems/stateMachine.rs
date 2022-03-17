use bevy::prelude::*;

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

#[derive(Debug)]
pub struct StateInfo {
    pub maxIndex: i16,
}

#[derive(Component, Debug, Default, Reflect, Copy, Clone)]
#[reflect(Component)]
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
        app.add_system(state_trigger).add_event::<StateChangeEvt>();
    }
}
fn state_trigger(
    mut stateChangeRead: EventReader<StateChangeEvt>,
    mut query: Query<&mut InsState>,
) {
    for ev in stateChangeRead.iter() {
        if let Ok(mut insState) = query.get_mut(ev.ins) {
            // println!(
            //     "发现该实体，并取得他的state :{:?} {:?}",
            //     state.0, ev.newState
            // );
            if (insState.0 != ev.newState) {
                // println!("你做到了");
                insState.0 = ev.newState;
            }

            println!(
                "curState:{:?} , curStateInfo:{:?}",
                insState.0,
                insState._get()
            );
        }
        // println!("ins :{:?} newState:{:?}", ev.ins, ev.newState)
    }
}
