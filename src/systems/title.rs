// // This example uses the LevelSet component instead of the LevelSelection resource.
// // The setup system puts every level uid in the LevelSet, so the entire LDtk world is spawned.

// use bevy::prelude::*;
// use bevy_ecs_ldtk::prelude::*;
// use std::collections::HashSet;

// use rand::prelude::*;

// pub struct TitlePlugin;
// impl Plugin for TitlePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugin(LdtkPlugin)
//             .add_startup_system(setup)
//             .add_system(toggle_levels)
//             .insert_resource(LdtkSettings {
//                 // By default, levels are just spawned at the origin of the world.
//                 // This makes them spawn according to their location in LDtk
//                 level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
//                     load_level_neighbors: false,
//                 },
//                 ..Default::default()
//             });
//     }
// }

// const LEVEL_IIDS: [&str; 8] = [
//     "a3591db0-66b0-11ec-9cd7-43878cf4d0ab",
//     "a35944c0-66b0-11ec-9cd7-6b4e2322a69e",
//     "a35992e0-66b0-11ec-9cd7-8b2ebd1b98e2",
//     "a359b9f0-66b0-11ec-9cd7-25dfb937d033",
//     "a35a2f20-66b0-11ec-9cd7-db6f994e2834",
//     "a35aa451-66b0-11ec-9cd7-438de356526d",
//     "a35acb61-66b0-11ec-9cd7-f76e35cfda30",
//     "a35b8eb0-66b0-11ec-9cd7-3d16ec48af10",
// ];

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let iids: HashSet<String> = LEVEL_IIDS.into_iter().map(|s| s.to_string()).collect();
//     commands.spawn_bundle(LdtkWorldBundle {
//         ldtk_handle: asset_server.load("WorldMap_Free_layout.ldtk"),
//         level_set: LevelSet { iids },
//         transform: Transform::from_xyz(-232., -496., 0.),
//         ..Default::default()
//     });
// }

// // This function is a demonstation that changes to the LevelSet have the expected results.
// // Hit spacebar and watch what happens!
// fn toggle_levels(input: Res<Input<KeyCode>>, mut level_sets: Query<&mut LevelSet>) {
//     if input.just_pressed(KeyCode::Space) {
//         let mut rng = rand::thread_rng();
//         let level_to_toggle = LEVEL_IIDS.choose(&mut rng).unwrap().to_string();

//         let mut level_set = level_sets.single_mut();
//         if level_set.iids.contains(&level_to_toggle) {
//             level_set.iids.remove(&level_to_toggle);
//         } else {
//             level_set.iids.insert(level_to_toggle);
//         }
//     }
// }
