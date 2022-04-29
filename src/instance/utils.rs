use crate::systems::collision::{CollisionBot, CollisionConfig, CollisionDynTag, CollisionStaTag};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn createDynCollision(commands: &mut Commands, x: f32, y: f32) -> Entity {
    // let shape = shapes::Rectangle {
    //     extents: Vec2::new(10.0, 10.0),
    //     origin: RectangleOrigin::Center,
    // };
    let collisionChildId = commands
        // .spawn_bundle(GeometryBuilder::build_as(
        //     &shape,
        //     DrawMode::Outlined {
        //         fill_mode: FillMode::color(Color::CYAN),
        //         outline_mode: StrokeMode::new(Color::BLACK, 1.0),
        //     },
        //     Transform::from_translation(Vec3::new(x, y, 0.0)),
        // ))
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            ..default()
        })
        .insert(CollisionDynTag)
        .insert(CollisionBot {
            pos: Vec2::new(x, y),
            force: Vec2::new(0.0, 0.0),
            wall_move: [None; 2],
        })
        .insert(Name::new("collision"))
        // .insert(Visibility { is_visible: false })
        .id();

    return collisionChildId;
}

pub fn createStaCollision(
    commands: &mut Commands,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Entity {
    // println!(
    //     "createStaCollision x = {}, y = {}, width = {}, height = {}",
    //     x, y, width, height
    // );
    let shape = shapes::Rectangle {
        extents: Vec2::new(width, height),
        origin: RectangleOrigin::Center,
    };
    let collisionChildId = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::from_translation(Vec3::new(x, y, 1.0)),
        ))
        .insert(CollisionStaTag)
        .insert(CollisionBot {
            pos: Vec2::new(x, y),
            force: Vec2::new(0.0, 0.0),
            wall_move: [None; 2],
        })
        .insert(CollisionConfig {
            width: width as i32,
            height: height as i32,
        })
        .insert(Name::new("collision"))
        // .insert(Visibility { is_visible: false })
        .id();

    return (collisionChildId);
}
