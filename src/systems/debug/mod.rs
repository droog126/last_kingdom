use crate::systems::debug::fps::FpsText;
use bevy::prelude::*;

pub mod collision;
pub mod egui;
pub mod fps;

pub struct DebugPlugin;
pub struct DebugStatus {
    pub fps_show: bool,
    pub camera_debug: bool,
    pub collision_debug: bool,
}
impl FromWorld for DebugStatus {
    fn from_world(world: &mut World) -> Self {
        DebugStatus {
            fps_show: true,
            camera_debug: false,
            collision_debug: false,
        }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // 热更新
        app.add_startup_system(setup)
            .init_resource::<DebugStatus>()
            .add_system_set(SystemSet::new().with_system(debug_switch));
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn debug_switch(
    input: Res<Input<KeyCode>>,
    mut debug_res: ResMut<DebugStatus>,
    mut query: Query<(&mut Visibility, With<FpsText>)>,
) {
    if (input.just_pressed(KeyCode::F11)) {
        debug_res.fps_show = !debug_res.fps_show;
        for (mut visibility, i) in query.iter_mut() {
            visibility.is_visible = !visibility.is_visible;
        }
    }

    if (input.just_pressed(KeyCode::F3)) {
        debug_res.camera_debug = !debug_res.camera_debug;
    }

    if (input.just_pressed(KeyCode::F12)) {
        debug_res.collision_debug = !debug_res.collision_debug;
    }
}
