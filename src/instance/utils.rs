use crate::systems::collision::CollisionTag;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn createCollision(commands: &mut Commands, x: f32, y: f32) -> Entity {
    println!("无效吗？");

    let shape = shapes::Rectangle {
        extents: Vec2::new(10.0, 10.0),
        origin: RectangleOrigin::Center,
    };
    let collisionChildId = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ))
        .insert(CollisionTag)
        .insert(Name::new("collision"))
        .insert(Visibility { is_visible: false })
        .id();

    return (collisionChildId);
}
