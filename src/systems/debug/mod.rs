use crate::systems::debug::fps::FpsText;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use self::egui::DebugTable;

pub mod collision;
pub mod egui;
pub mod fps;

pub struct DebugPlugin;
pub struct DebugStatus {
    pub fps_show: bool,
    pub camera_debug: bool,
    pub collision_debug: bool,
    pub debug_info: bool,
}
impl FromWorld for DebugStatus {
    fn from_world(world: &mut World) -> Self {
        DebugStatus {
            fps_show: true,
            camera_debug: false,
            collision_debug: false,
            debug_info: false,
        }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // 热更新
        app.add_startup_system(startup)
            .init_resource::<DebugStatus>()
            .add_system_set(SystemSet::new().with_system(debug_switch));

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(egui::EGuiPlugin)
                .add_plugin(fps::FpsPlugin)
                .add_plugin(collision::CollisionDebugPlugin);
        }
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DebugTable {
        fps: None,
        collisionCount: None,
    });
}

fn debug_switch(
    input: Res<Input<KeyCode>>,
    mut debugStatus: ResMut<DebugStatus>,
    mut query: Query<(&mut Visibility, With<FpsText>)>,
) {
    if (input.just_pressed(KeyCode::F11)) {
        debugStatus.fps_show = !debugStatus.fps_show;
        debugStatus.debug_info = !debugStatus.debug_info;

        for (mut visibility, i) in query.iter_mut() {
            visibility.is_visible = !visibility.is_visible;
        }
    }

    if (input.just_pressed(KeyCode::F3)) {
        debugStatus.camera_debug = !debugStatus.camera_debug;
    }

    if (input.just_pressed(KeyCode::F12)) {
        debugStatus.collision_debug = !debugStatus.collision_debug;
    }
}
