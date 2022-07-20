use std::fmt::{self, Formatter};

use bevy::prelude::*;

use super::animation::{AnimationInfo, AnimationValue, SpriteConfigFn};

pub fn create_instance(commands: &mut Commands, imageHandle: Handle<Image>, x: f32, y: f32) {}
#[derive(Debug, Clone)]
pub struct StaBasic {
    x: f32,
    y: f32,
    imageName: String,
}
#[derive(Clone)]
pub struct DynBasic {
    x: f32,
    y: f32,
    spriteConfig: SpriteConfigFn,
}
impl fmt::Debug for DynBasic {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "DynBasic")
    }
}

#[derive(Debug, Clone)]
pub struct StaInstance {
    x: f32,
    y: f32,
    imageName: String,
}

#[derive(Clone)]
pub struct DynInstance {
    x: f32,
    y: f32,
    spriteConfig: SpriteConfigFn,
}
impl fmt::Debug for DynInstance {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "DynInstance")
    }
}

#[derive(Debug, Clone)]
pub enum CreateInstanceEnum {
    StaBasic(StaBasic),
    DynBasic(DynBasic),
    StaInstance(StaInstance),
    DynInstance(DynInstance),
}

#[derive(Clone, Debug, Component)]
pub struct CreateInstanceEvent(CreateInstanceEnum);

pub fn factory_step(commands: &mut Commands) {}

pub fn factory() {}
