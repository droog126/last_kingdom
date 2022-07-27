use std::fmt::{self, Formatter};

use bevy::{pbr::Shadow, prelude::*, sprite::Anchor};

use crate::{
    state::loading::{ImageCenter, TextureAtlasCenter},
    systems::{
        input::InsInput,
        instance::{animation::AnimationMachine, basicCreate::create_dyn_collision, iType::player::PlayerAnimationTag},
    },
};

use super::{
    animation::{AnimationInfo, AnimationValue, SpriteConfigFn},
    basicCreate::{create_shadow, create_sta_collision},
    iType::{
        player::{CollisionBoxExcludeFunction, PlayerTag},
        InstanceCamp, InstanceType,
    },
    props::InstanceProps,
};

#[derive(Clone)]
pub enum ExtInstanceParam {
    Player {},
    HostPlayer {},
    Snake {},
    None,
}

#[derive(Component)]
pub struct DynInstanceTag;

#[derive(Component)]
pub struct StaInstanceTag;

#[derive(Component)]
pub struct StaBasicTag;

#[derive(Component)]
pub struct DynBasicTag;

#[derive(Clone)]
pub enum CreateInstanceEnum {
    StaBasic {
        x: f32,
        y: f32,
        imageName: String,
    },
    DynBasic {
        x: f32,
        y: f32,
        spriteConfig: SpriteConfigFn,
    },
    StaInstance {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        name: Name,
        imageName: String,
        instanceProps: InstanceProps,
        collisionExcludeFunction: Option<CollisionBoxExcludeFunction>,
        instanceType: InstanceType,
        instanceCamp: InstanceCamp,
    },
    DynInstance {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        name: Name,
        spriteName: String,
        spriteConfig: SpriteConfigFn,
        spriteOffset: Vec2,
        instanceProps: InstanceProps,
        collisionExcludeFunction: Option<CollisionBoxExcludeFunction>,
        instanceType: InstanceType,
        instanceCamp: InstanceCamp,
        ext: ExtInstanceParam,
    },
}
// DebugTag
impl fmt::Debug for CreateInstanceEnum {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "DynInstance")
    }
}
#[derive(Clone, Debug, Component)]
pub struct CreateInstanceEvent(pub CreateInstanceEnum);

pub fn factory_create(app: &mut bevy::prelude::App) {
    app.add_event::<CreateInstanceEvent>();
    app.add_stage_after(CoreStage::Update, "createInstance", SystemStage::parallel())
        .add_system_to_stage("createInstance", factory_step);
}

pub fn factory_step(
    mut createInstanceEvent: EventReader<CreateInstanceEvent>,
    mut commands: Commands,
    textureAtlasCenter: Res<TextureAtlasCenter>,
    imageCenter: Res<ImageCenter>,
) {
    for evt in createInstanceEvent.iter() {
        match evt.0.clone() {
            CreateInstanceEnum::StaBasic { x, y, imageName } => {
                commands.spawn_bundle(SpriteBundle {
                    texture: imageCenter.0.get(&imageName).unwrap().clone(),
                    // 精灵锚点
                    sprite: Sprite { anchor: Anchor::Center, ..default() },
                    ..default()
                });
            }
            CreateInstanceEnum::DynBasic { x, y, spriteConfig } => {}
            CreateInstanceEnum::StaInstance {
                x,
                y,
                imageName,
                width,
                height,
                instanceProps,
                collisionExcludeFunction,
                instanceType,
                instanceCamp,
                name,
            } => {
                // 阴影实体
                let shadowId = create_shadow(&mut commands, &imageCenter, width, height);

                let staSpriteId = commands
                    .spawn()
                    .insert_bundle(SpriteBundle {
                        texture: imageCenter.0.get(&imageName).unwrap().clone(),
                        // 精灵锚点
                        sprite: Sprite { anchor: Anchor::BottomCenter, ..default() },
                        ..default()
                    })
                    .id();

                // 人物实体
                let instanceId = create_sta_collision(
                    &mut commands,
                    x,
                    y,
                    width,
                    height,
                    instanceType,
                    instanceCamp,
                    collisionExcludeFunction,
                );
                let mut instance = commands.entity(instanceId);
                instance.insert(name).insert(StaInstanceTag).insert(instanceProps);
                instance.push_children(&[shadowId, staSpriteId]);
            }
            CreateInstanceEnum::DynInstance {
                x,
                y,
                spriteName,
                spriteConfig,
                width,
                height,
                instanceProps,
                collisionExcludeFunction,
                instanceType,
                instanceCamp,
                name,
                ext,
                spriteOffset,
            } => {
                // 阴影实体
                let shadowId = create_shadow(&mut commands, &imageCenter, width, height);

                //  动画实体
                let animationId = commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite { anchor: Anchor::BottomCenter, ..default() },
                        transform: Transform {
                            translation: Vec3::new(spriteOffset.x, spriteOffset.y, -1.0),
                            ..Default::default()
                        },
                        texture_atlas: textureAtlasCenter.0.get(&spriteName).unwrap().clone(),
                        ..Default::default()
                    })
                    .insert(AnimationMachine { value: AnimationValue::Idle, progress: 0.0, config: spriteConfig })
                    .insert(Name::new("animation".to_string()))
                    .insert(PlayerAnimationTag)
                    .id();

                // 人物实体
                let instanceId = create_dyn_collision(
                    &mut commands,
                    x,
                    y,
                    width,
                    height,
                    instanceType,
                    instanceCamp,
                    collisionExcludeFunction,
                );
                let mut instance = commands.entity(instanceId);
                instance.insert(name).insert(DynInstanceTag).insert(instanceProps);
                instance.push_children(&[animationId, shadowId]);

                match ext {
                    ExtInstanceParam::Player {} => {}
                    ExtInstanceParam::Snake {} => {}
                    ExtInstanceParam::None => {}
                    ExtInstanceParam::HostPlayer {} => {
                        instance.insert(InsInput { ..Default::default() });
                        instance.insert(PlayerTag);
                    }
                }
            }
        }
    }
}

// toDo 位置拼接
