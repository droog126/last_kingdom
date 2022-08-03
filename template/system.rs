use bevy::prelude::*;
pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup).add_system(step)
    }
}
fn startup(mut commands: Commands) {}
fn step(mut commands: Commands) {}

//  计划
use bevy::{ecs::schedule::ShouldRun, prelude::*};

use super::DebugStatus;
pub struct CollisionDebugPlugin;
impl Plugin for CollisionDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup)
            .add_system(step)
            .add_system_set(SystemSet::new().with_system(step).with_run_criteria(need_run));
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

fn step(mut commands: Commands, debugStatus: Res<DebugStatus>) {}

// 触发器

use crate::systems::collision::CollisionDynTag;
use bevy::{ecs::schedule::ShouldRun, prelude::*};
use rand::prelude::*;

use super::DebugStatus;
pub struct CollisionDebugPlugin;
impl Plugin for CollisionDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup)
            // .add_system(step)
            .add_system_set(SystemSet::new().with_system(trigger).with_run_criteria(need_run));
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

fn trigger(mut commands: Commands, query: Query<Entity, With<CollisionDynTag>>, mut debugStatus: ResMut<DebugStatus>) {
    let shape = shapes::Rectangle { extents: Vec2::new(20.0, 10.0), origin: RectangleOrigin::Center };

    for entity in query.iter() {
        commands.entity(entity).with_children(|child| {
            child.spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::CYAN),
                    outline_mode: StrokeMode::new(Color::BLACK, 1.0),
                },
                Transform::default(),
            ));
        });
    }

    debugStatus.collision_debug = false;
}
