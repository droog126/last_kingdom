use crate::systems;
use crate::systems::instance::attack::attack_event_distribution_system;
use crate::systems::instance::factory::{
    factory_create, factory_step, CreateInstanceEnum, CreateInstanceEvent, ExtInstanceParam,
};
use crate::systems::instance::iType::player::{getPlayerSprite, playerCollisionExclude, player_create};
use crate::systems::instance::iType::snake::{getSnakeSprite, snakeCollisionExclude, snake_create, snake_step};
use crate::systems::instance::iType::{InstanceCamp, InstanceType};
use crate::systems::instance::props::{BasicProps, InstanceProps};
use crate::systems::instance::z_depth_step;
use crate::systems::item::{twoHand_create, twoHand_step};
use crate::utils::random::random_range;
use crate::{state::GameState, systems::instance::iType::player::player_step};
// use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::loading::{ImageCenter, TextureAtlasCenter};

pub fn playing_start(app: &mut bevy::prelude::App) {
    app.add_plugin(systems::instance::collision::CollisionPlugin)
        .add_plugin(systems::instance::animation::AnimationPlugin);
    factory_create(app);
    // plugins

    app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(playing_enter))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                // 运行系统
                .with_system(playing_setup)
                .with_system(attack_event_distribution_system)
                .with_system(z_depth_step)
                // 实体系统
                .with_system(player_step)
                .with_system(snake_step)
                .with_system(twoHand_step),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(playing_exit));
}

fn playing_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    textureAtlasCenter: Res<TextureAtlasCenter>,
    imageCenter: Res<ImageCenter>,
    mut createInstance: EventWriter<CreateInstanceEvent>,
) {
    createInstance.send(CreateInstanceEvent(CreateInstanceEnum::DynInstance {
        x: 20.0,
        y: 20.0,
        spriteName: "player".to_string(),
        spriteConfig: getPlayerSprite,
        width: 10.0,
        height: 10.0,
        instanceProps: InstanceProps::new(BasicProps {
            hp: 20.,
            energy: 20.,
            speed: 200.,
            bouncing: 400.,
            maxHp: 20.,
            maxEnergy: 20.,
            maxSpeed: 200.,
            maxBouncing: 400.,
        }),
        collisionExcludeFunction: Some(playerCollisionExclude),
        instanceType: InstanceType::Player,
        instanceCamp: InstanceCamp::Friendly,
        name: Name::new("player"),
        ext: ExtInstanceParam::HostPlayer {},
        spriteOffset: Vec2::new(0.0, -4.0),
    }));

    createInstance.send(CreateInstanceEvent(CreateInstanceEnum::StaBasic {
        x: 0.0,
        y: 0.0,
        imageName: "map".to_string(),
    }));

    createInstance.send(CreateInstanceEvent(CreateInstanceEnum::DynInstance {
        x: 20.0,
        y: 20.0,
        spriteName: "snake".to_string(),
        spriteConfig: getSnakeSprite,
        width: 10.0,
        height: 10.0,
        instanceProps: InstanceProps::new(BasicProps {
            hp: 20.,
            energy: 20.,
            speed: 200.,
            bouncing: 400.,
            maxHp: 20.,
            maxEnergy: 20.,
            maxSpeed: 200.,
            maxBouncing: 400.,
        }),
        collisionExcludeFunction: Some(snakeCollisionExclude),
        instanceType: InstanceType::Snake,
        instanceCamp: InstanceCamp::Hostile,
        name: Name::new("snake"),
        ext: ExtInstanceParam::Snake {},
        spriteOffset: Vec2::new(4.0, -1.5),
    }));

    createInstance.send(CreateInstanceEvent(CreateInstanceEnum::StaInstance {
        x: random_range(-100.0, 100.0),
        y: random_range(-100.0, 100.0),
        width: 20.0,
        height: 20.0,
        name: Name::new("tree"),
        imageName: "tree1".to_string(),
        instanceProps: InstanceProps::new(BasicProps {
            hp: 20.,
            energy: 20.,
            speed: 200.,
            bouncing: 400.,
            maxHp: 20.,
            maxEnergy: 20.,
            maxSpeed: 200.,
            maxBouncing: 400.,
        }),
        collisionExcludeFunction: None,
        instanceType: InstanceType::Tree,
        instanceCamp: InstanceCamp::Neutral,
    }));

    createInstance.send(CreateInstanceEvent(CreateInstanceEnum::StaInstance {
        x: random_range(-100.0, 100.0),
        y: random_range(-100.0, 100.0),
        width: 20.0,
        height: 20.0,
        name: Name::new("tree"),
        imageName: "tree2".to_string(),
        instanceProps: InstanceProps::new(BasicProps {
            hp: 20.,
            energy: 20.,
            speed: 200.,
            bouncing: 400.,
            maxHp: 20.,
            maxEnergy: 20.,
            maxSpeed: 200.,
            maxBouncing: 400.,
        }),
        collisionExcludeFunction: None,
        instanceType: InstanceType::Tree,
        instanceCamp: InstanceCamp::Neutral,
    }));
}

fn playing_setup() {
    // println!("游戏进行中")
}

fn playing_exit() {}
