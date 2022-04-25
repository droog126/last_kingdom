use crate::{
    instance::utils::createDynCollision,
    systems::{camera::CursorPosition, collision::CollisionDynTag},
    utils::random::random_xy,
};
use bevy::{ecs::schedule::ShouldRun, prelude::*};

use super::DebugStatus;
pub struct CollisionDebugPlugin;
impl Plugin for CollisionDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup).add_system_set(
            SystemSet::new()
                .with_system(trigger)
                .with_system(step)
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

fn trigger(
    mut commands: Commands,
    mut query: Query<&mut Visibility, With<CollisionDynTag>>,
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

fn step(
    mouseInput: Res<Input<MouseButton>>,
    cursorPoint: Res<CursorPosition>,
    mut commands: Commands,
) {
    if mouseInput.just_pressed(MouseButton::Middle) {
        let mut ids = random_xy(1000, 1000)
            .take(1000)
            .map(|[x, y]| createDynCollision(&mut commands, x, y))
            .collect::<Vec<_>>();
    }
}
