use bevy::prelude::*;

use crate::{
    // instance::snake::snake_create_raw,
    state::loading::SpriteCenter,
    systems::{camera::CursorPosition, instance::shadow::ShadowAsset},
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

    mut spriteCenter: ResMut<SpriteCenter>,
    shadowHandle: Res<ShadowAsset>,
) {
    if (!debugStatus.instance_debug) {
        return;
    }
    if (mouseInput.just_pressed(MouseButton::Left)) {
        println!("create snake{:?}", cursorPosition);

        // snake_create_raw(
        //     &mut commands,
        //     spriteCenter,
        //     shadowHandle,
        //     cursorPosition.x,
        //     cursorPosition.y,
        // );
    }
}
