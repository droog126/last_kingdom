// 画一个圆
// let shape = shapes::Circle {
//     radius: 16.,
//     center: Vec2::new(0., 0.),
// };

// commands
//     .spawn_bundle(GeometryBuilder::build_as(
//         &shape,
//         DrawMode::Outlined {
//             fill_mode: FillMode::color(Color::CYAN),
//             outline_mode: StrokeMode::new(Color::BLACK, 1.0),
//         },
//         Transform::default(),
//     ))
//     .insert(Name::new("collision".to_string()));


//Vec2 
``` rust
 Vec2::splat(20.0),

```

// res
``` rust
input: Res<Input<KeyCode>>,
if (input.just_pressed(KeyCode::F11)) {}


fn mouse_click_system(mouse_button_input: Res<Input<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        info!("left mouse currently pressed");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}


```


// 通过查询找到实例 
```rust
fn step(mut commands: Commands, query: Query<Entity, With<CollisionDynTag>>) {
    for entity in query.iter() {
        commands.entity(entity)
    }
}

```


// 创建一个攻击盒子 
```rust
create_attack_box(
        shadowHandle.clone(),
        textureAtlasCenter.0.get("snake").unwrap().clone(),
        getSnakeSprite,
        &mut commands,
        "_snake",
        SnakeTag,
        InstanceType::Snake,
        InstanceCamp::Hostile,
        None,
        pos.x,
        pos.y,
        20.,
        20.,
    );
```