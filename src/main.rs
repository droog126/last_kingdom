#![allow(non_snake_case)]
// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use last_kingdom::GamePlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "LastKingdom".to_string(), // ToDo
            vsync: false,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin);
    app.run();
}
