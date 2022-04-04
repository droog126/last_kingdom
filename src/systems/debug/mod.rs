use crate::systems::debug::fps::FpsText;
use bevy::prelude::*;

pub mod egui;
pub mod fps;

pub struct DebugPlugin;
pub struct DebugControl {
    pub fps_show: bool,
    pub camera_debug: bool,
}
impl FromWorld for DebugControl {
    fn from_world(world: &mut World) -> Self {
        DebugControl {
            fps_show: true,
            camera_debug: false,
        }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // 热更新
        app.add_startup_system(setup)
            .init_resource::<DebugControl>()
            .add_system_set(SystemSet::new().with_system(debug_switch));
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // #[cfg(debug_assertions)]
    // asset_server.watch_for_changes().unwrap();
}

fn debug_switch(
    input: Res<Input<KeyCode>>,
    mut debug_res: ResMut<DebugControl>,
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
}
