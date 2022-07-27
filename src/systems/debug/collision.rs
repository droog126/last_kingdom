use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::systems::instance::factory::{DynInstanceTag, StaInstanceTag};

use super::DebugStatus;

pub fn collision_debug_create(app: &mut App) {
    app.add_system_set(SystemSet::new().with_system(show_collision).with_run_criteria(if_show_collision));
}
pub fn if_show_collision(debugStatus: Res<DebugStatus>) -> ShouldRun {
    if debugStatus.collision_debug {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn show_collision(
    mut query: Query<(&mut Visibility, AnyOf<(&DynInstanceTag, &StaInstanceTag)>)>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Key1) {
        for (mut visible, _) in query.iter_mut() {
            visible.is_visible = !visible.is_visible;
        }
    }
}
