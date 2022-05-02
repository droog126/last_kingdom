#![allow(non_snake_case)]
// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use last_kingdom::config::Config;
use last_kingdom::GamePlugin;

fn main() {
    let config = Config::new("config.ini");

    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: config.width(),
            height: config.height(),
            title: "LastKingdom".to_string(), // ToDo
            present_mode: PresentMode::Immediate,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin);
    app.run();
}
