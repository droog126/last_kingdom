use crate::systems::collision::{CollisionBot, CollisionInner};
use crate::utils::num::y_to_z;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn create_instance_collision(
    commands: &mut Commands,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Entity {
    let collisionId = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, y_to_z(y))),
            ..default()
        })
        .insert(Name::new("collision"))
        // .insert(Visibility { is_visible: false })
        .id();

    commands.entity(collisionId).insert(CollisionBot {
        collisionInner: CollisionInner::Instance {
            pos: Vec2::new(x, y),
            force: Vec2::new(0.0, 0.0),
            wall_move: [None, None],
        },
        width: width,
        height: height,
        id: collisionId,
    });
    return collisionId;
}

pub fn create_sta_collision(
    commands: &mut Commands,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Entity {
    let collisionId = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.4, 0.4, 0.4),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, y_to_z(y))),
            ..default()
        })
        .insert(Name::new("staCollision"))
        // .insert(Visibility { is_visible: false })
        .id();
    commands.entity(collisionId).insert(CollisionBot {
        id: collisionId,
        width: width,
        height: height,
        collisionInner: CollisionInner::Static,
    });

    return collisionId;
}

pub fn create_scope_collision(
    commands: &mut Commands,
    parentId: Entity,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Entity {
    let collisionId = commands
        .spawn()
        .insert(GlobalTransform { ..default() })
        .insert(Transform {
            translation: Vec3::new(x, y, 100.0 - y / 10000.0),
            ..default()
        })
        .id();

    commands.entity(collisionId).insert(CollisionBot {
        collisionInner: CollisionInner::Scope { other: vec![] },
        width: width,
        height: height,
        id: collisionId,
    });
    return collisionId;
}
