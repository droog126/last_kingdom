use crate::{instance::snake::snake_create_raw, systems::debug::DebugStatus};
use bevy::prelude::*;

pub fn exclusive_system_debug(world: &mut World) {
    // this does the same thing as "new_player_system"
    // let total_players = world.resource_mut::<GameState>().total_players;
    // let should_add_player = {
    //     let game_rules = world.resource::<GameRules>();
    //     let add_new_player = random::<bool>();
    //     add_new_player && total_players < game_rules.max_players
    // };
    // // Randomly add a new player
    // if should_add_player {
    //     world.spawn().insert_bundle((
    //         Player {
    //             name: format!("Player {}", total_players),
    //         },
    //         Score { value: 0 },
    //     ));

    //     let mut game_state = world.resource_mut::<GameState>();
    //     game_state.total_players += 1;
    // }
}
