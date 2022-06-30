use bevy::prelude::*;

use crate::{
    state::loading::{ImageCenter, TextureAtlasCenter},
    utils::num::y_to_z,
};

use super::{
    camera::{CursorDiff, CursorPosition},
    instance::{
        animation::{AnimationMachine, AnimationValue, StateInfo},
        attack::{create_attack_box, AttackEventPart, RepelData},
        basicCreate::create_instance_collision,
        instanceType::{player::PlayerTag, InstanceCamp, InstanceType},
    },
    timeLine::TimeLine,
};
// 标识实体
#[derive(Component)]
pub struct ItemTag;

#[derive(Component)]
pub struct TwoHandTag;

#[derive(Component)]
pub struct TwoHandConfig {
    pub host: Entity,
    pub damage: f32,
    pub cd: i32,
    pub nextTime: i32,
}

fn get_twoHand_sprite(animationValue: &AnimationValue) -> StateInfo {
    match *animationValue {
        AnimationValue::Idle => StateInfo { startIndex: 0, endIndex: 0, spriteName: "twoHand".to_string() },
        AnimationValue::Walk => StateInfo { startIndex: 8, endIndex: 15, spriteName: "twoHand".to_string() },
        _ => StateInfo { startIndex: 0, endIndex: 0, spriteName: "twoHand".to_string() },
    }
}
pub fn twoHand_create(
    mut commands: &mut Commands,
    textureAtlasCenter: &Res<TextureAtlasCenter>,
    imageCenter: &Res<ImageCenter>,
    hostId: Entity,
    x: f32,
    y: f32,
) {
    // Item实体
    let animationId = commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform { translation: Vec3::new(0.0, 20.0, 10.0), ..Default::default() },
            texture_atlas: textureAtlasCenter.0.get("twoHand").unwrap().clone(),
            ..Default::default()
        })
        .insert(AnimationMachine { value: AnimationValue::Idle, progress: 0.0, config: get_twoHand_sprite })
        .insert(Name::new("hand".to_string()))
        .insert(TwoHandTag)
        .insert(TwoHandConfig { host: hostId, damage: 2., cd: 10, nextTime: 0 })
        .id();
}

pub fn twoHand_step(
    mut commands: Commands,
    mut localY: Local<f32>,
    mut query: Query<(&mut Transform, &mut TwoHandConfig), With<TwoHandTag>>,
    mut hostQuery: Query<(&Transform, &Sprite), (With<PlayerTag>, Without<TwoHandTag>)>,
    keyInput: Res<Input<KeyCode>>,
    mouseInput: Res<Input<MouseButton>>,
    cursorDiff: Res<CursorDiff>,
    timeLine: Res<TimeLine>,
    imageCenter: Res<ImageCenter>,
) {
    let timeLineRaw = timeLine.0;
    for (mut selfTransform, mut twoHandConfig) in query.iter_mut() {
        #[cfg(debug_assertions)]
        {
            if (keyInput.pressed(KeyCode::Up)) {
                *localY += 0.1;
            }
            if (keyInput.pressed(KeyCode::Down)) {
                *localY -= 0.1;
            }
        }

        selfTransform.translation.z = y_to_z(selfTransform.translation.y) + 20.0;
        if let Ok((hostTransform, hostSprite)) = hostQuery.get(twoHandConfig.host) {
            selfTransform.translation.x = hostTransform.translation.x;
            selfTransform.translation.y = hostTransform.translation.y + 7.0;

            selfTransform.translation.x += cursorDiff.0.x * 8.0;
            selfTransform.translation.y += cursorDiff.0.y * 5.0;
        }

        if (twoHandConfig.nextTime < timeLineRaw) {
            if mouseInput.just_pressed(MouseButton::Left) {
                create_attack_box(
                    &mut commands,
                    imageCenter.0.get("circle").unwrap().clone(),
                    InstanceType::Player,
                    InstanceCamp::Friendly,
                    Some(|instanceType, collisionType, instanceCamp| {
                        if (instanceType != &InstanceType::Player) {
                            false
                        } else {
                            true
                        }
                    }),
                    AttackEventPart {
                        damage: twoHandConfig.damage,
                        nextTime: timeLineRaw + 20,
                        repelData: Some(RepelData { dif: cursorDiff.0 * 60.0, timeLen: 20 }),
                    },
                    selfTransform.translation.x,
                    selfTransform.translation.y,
                    20.,
                    20.,
                );

                twoHandConfig.nextTime = timeLineRaw + twoHandConfig.cd;
            }
        }
        // println!("{:?}", twoHandConfig.nextTime);
        //todo 杀一只蛇
    }
}
