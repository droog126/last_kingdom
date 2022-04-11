use bevy::prelude::*;

use crate::instance::player::PlayerTag;

use super::{debug::DebugControl, input::InsInput};

#[derive(Component)]
pub struct MainCameraTag;

pub struct CursorPosition {
    x: f32,
    y: f32,
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_create)
            .insert_resource(CursorPosition { x: 0.0, y: 0.0 })
            .add_system(camera_step);
    }
}

fn camera_create(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.scale.x = 0.5;
    camera.transform.scale.y = 0.5;
    commands.spawn_bundle(camera).insert(MainCameraTag);
}

fn camera_step(
    wnds: Res<Windows>,
    time: Res<Time>,
    mut cursorPosition: ResMut<CursorPosition>,
    debugStatus: Res<DebugControl>,
    mut query: QuerySet<(
        QueryState<(&Camera, &mut Transform), With<MainCameraTag>>,
        QueryState<(&InsInput, &GlobalTransform), With<PlayerTag>>,
    )>,
) {
    let mut dir = None;
    let mut playerPosition = None;

    for (insInput, playerTransform) in query.q1().iter() {
        dir = Some(insInput.dir.clone());
        playerPosition = Some(playerTransform.translation);
    }

    // 捕获鼠标在Camera的坐标
    for (camera, mut camera_transform) in query.q0().iter_mut() {
        let wnd = wnds.get(camera.window).unwrap();

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix.inverse();

            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            let world_pos: Vec2 = world_pos.truncate();
            cursorPosition.x = world_pos.x;
            cursorPosition.y = world_pos.y;
        }

        // 任务:debug控制相机
        if (debugStatus.camera_debug && dir != None) {
            let unwrapDir = dir.unwrap();
            camera_transform.translation.x += unwrapDir.x;
            camera_transform.translation.y += unwrapDir.y;
        }

        // 任务:跟随玩家
        if (!debugStatus.camera_debug && playerPosition != None) {
            let mut unwrapPlayerPosition = playerPosition.unwrap();
            unwrapPlayerPosition.z = camera_transform.translation.z;

            let diff = camera_transform.translation - unwrapPlayerPosition;

            // println!("diff:{:?}", diff);
            //1/4秒回到目标身上
            let factor = time.delta_seconds() * 4.0;

            if diff.length() <= factor {
                camera_transform.translation = unwrapPlayerPosition;
            } else {
                camera_transform.translation -= diff * factor;
            }
        }
    }
}