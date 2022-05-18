use bevy::{ecs::schedule::ShouldRun, prelude::*};

use super::DebugStatus;
pub struct CollisionDebugPlugin;
impl Plugin for CollisionDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup).add_system_set(
            SystemSet::new()
                .with_system(first)
                .with_run_criteria(need_run),
        );
    }
}

fn need_run(debugStatus: Res<DebugStatus>) -> ShouldRun {
    if (debugStatus.collision_debug) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn startup(mut commands: Commands) {}

fn first(
    mut commands: Commands,
    mut query: Query<&mut Visibility>,
    mut debugStatus: ResMut<DebugStatus>,
    mut local: Local<bool>,
) {
    if (*local != debugStatus.collision_debug) {
        *local = debugStatus.collision_debug;
        for mut visible in query.iter_mut() {
            visible.is_visible = debugStatus.collision_debug;
        }
    }
}
