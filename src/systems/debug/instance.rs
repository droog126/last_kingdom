use bevy::prelude::*;

use crate::{
    state::loading::{ImageCenter, TextureAtlasCenter},
    // instance::snake::snake_create_raw,
    systems::{camera::CursorPosition, instance::iType::snake::snake_create},
    utils::random::{random_arr4, random_range},
};

use super::DebugStatus;
pub struct InstanceDebugPlugin;
impl Plugin for InstanceDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup).add_system(step);
    }
}
fn startup(mut commands: Commands) {}
fn step(
    mut commands: Commands,
    mouseInput: Res<Input<MouseButton>>,
    cursorPosition: Res<CursorPosition>,
    debugStatus: Res<DebugStatus>,
    textureAtlasCenter: Res<TextureAtlasCenter>,
    imageCenter: Res<ImageCenter>,
) {
    if (!debugStatus.instance_debug) {
        return;
    }
    if (mouseInput.just_pressed(MouseButton::Left)) {
        // snake_create(
        //     &mut commands,
        //     textureAtlasCenter,
        //     imageCenter,
        //     cursorPosition.x,
        //     cursorPosition.y,
        // );
        // println!("create snake{:?}", cursorPosition);
        for _ in 0..10000 {
            snake_create(
                &mut commands,
                &textureAtlasCenter,
                &imageCenter,
                cursorPosition.x + random_range(-1000.0, 1000.0),
                cursorPosition.y + random_range(-1000.0, 1000.0),
            );
        }
    }

    // if mouseInput.just_pressed(MouseButton::Right) {
    //     let mut ids = random_arr4(1000, 1000, 100, 100)
    //         .take(2)
    //         .map(|[x, y, width, height]| {
    //             create_sta_collision(
    //                 &mut commands,
    //                 x as f32,
    //                 y as f32,
    //                 width as f32,
    //                 height as f32,
    //             )
    //         })
    //         .collect::<Vec<_>>();
    // }
}
