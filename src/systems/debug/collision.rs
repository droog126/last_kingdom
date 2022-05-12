use crate::{
    instance::{
        utils::{create_instance_collision, create_sta_collision},
        InstanceType,
    },
    // instance::utils::{createDynCollision, createStaCollision},
    systems::{camera::CursorPosition, collision::CollisionProductionFactor},
    utils::random::{random_arr2, random_arr4},
};
use bevy::{ecs::schedule::ShouldRun, prelude::*};

use super::DebugStatus;
pub struct CollisionDebugPlugin;
impl Plugin for CollisionDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup).add_system_set(
            SystemSet::new()
                .with_system(first)
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

fn first(
    mut commands: Commands,
    mut query: Query<&mut Visibility, With<CollisionProductionFactor>>,
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
        let mut ids = random_arr2(1000, 1000)
            .take(1000)
            .map(|[x, y]| {
                create_instance_collision(&mut commands, InstanceType::Player, x, y, 10.0, 10.0)
            })
            .collect::<Vec<_>>();
    }

    if mouseInput.just_pressed(MouseButton::Right) {
        let mut ids = random_arr4(1000, 1000, 100, 100)
            .take(2)
            .map(|[x, y, width, height]| {
                create_sta_collision(
                    &mut commands,
                    x as f32,
                    y as f32,
                    width as f32,
                    height as f32,
                )
            })
            .collect::<Vec<_>>();
    }
}
