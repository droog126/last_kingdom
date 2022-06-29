use bevy::{prelude::Component, utils::HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};

use crossbeam_channel::{bounded, Receiver, Sender};

use crate::utils::channel::if_channels_has_message;
// 问题: 玩家装备武器时，获得属性刷新。
// 需要get 和 set
// 武器升级时，-> 玩家属性刷新，武器属性刷新。
#[derive(Clone, Debug, Copy)]
pub struct BasicProps {
    pub hp: f32,
    pub energy: f32,
    pub speed: f32,
    pub bouncing: f32,
    pub maxHp: f32,
    pub maxEnergy: f32,
    pub maxSpeed: f32,
    pub maxBouncing: f32,
}

#[derive(Clone, Debug, Copy)]
pub struct RealProps {
    pub hp: f32,
    pub energy: f32,
    pub speed: f32,
    pub bouncing: f32,
    pub maxHp: f32,
    pub maxEnergy: f32,
    pub maxSpeed: f32,
    pub maxBouncing: f32,
}
impl From<BasicProps> for RealProps {
    fn from(basic: BasicProps) -> Self {
        RealProps {
            hp: basic.hp,
            energy: basic.energy,
            speed: basic.speed,
            bouncing: basic.bouncing,
            maxHp: basic.maxHp,
            maxEnergy: basic.maxEnergy,
            maxSpeed: basic.maxSpeed,
            maxBouncing: basic.maxBouncing,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ItemPropsSendData {
    pub id: usize,
    pub realProps: RealProps,
}

// 设计数值
#[derive(Debug, Component)]
pub struct InstanceProps {
    basicProps: BasicProps,
    realProps: RealProps,
    receivers: HashMap<usize, Receiver<ItemPropsSendData>>,
    storeReceiver: HashMap<usize, RealProps>,
}
impl InstanceProps {
    pub fn new(basicProps: BasicProps) -> InstanceProps {
        InstanceProps {
            basicProps,
            realProps: RealProps::from(basicProps),
            receivers: HashMap::new(),
            storeReceiver: HashMap::new(),
        }
    }
    pub fn get(&mut self) -> &RealProps {
        // 消费上游的装备更新事件
        if if_channels_has_message(&self.receivers) {
            self.update();
        }
        &self.realProps
    }
    pub fn add_item(&mut self, itemProps: &ItemProps) {
        itemProps.update_props();
        self.receivers.insert(itemProps.id, itemProps.receiver.clone());
        self.update();
    }
    pub fn update(&mut self) {
        for receiver in self.receivers.values() {
            if let Ok(itemPropsSendData) = receiver.try_recv() {
                self.storeReceiver.insert(itemPropsSendData.id, itemPropsSendData.realProps.clone());
            }
        }
        self.realProps.hp = self.basicProps.hp;
        self.realProps.energy = self.basicProps.energy;
        self.realProps.speed = self.basicProps.speed;
        self.realProps.bouncing = self.basicProps.bouncing;
        self.realProps.maxHp = self.basicProps.maxHp;
        self.realProps.maxEnergy = self.basicProps.maxEnergy;
        self.realProps.maxSpeed = self.basicProps.maxSpeed;
        self.realProps.maxBouncing = self.basicProps.maxBouncing;

        for itemProps in self.storeReceiver.values() {
            self.realProps.hp += itemProps.hp;
            self.realProps.energy += itemProps.energy;
            self.realProps.speed += itemProps.speed;
            self.realProps.bouncing += itemProps.bouncing;
            self.realProps.maxHp += itemProps.maxHp;
            self.realProps.maxEnergy += itemProps.maxEnergy;
            self.realProps.maxSpeed += itemProps.maxSpeed;
            self.realProps.maxBouncing += itemProps.maxBouncing;
        }
    }

    pub fn remove_item(&mut self, itemId: usize) {
        self.receivers.remove(&itemId);
        self.storeReceiver.remove(&itemId);
        self.update();
    }

    pub fn sub_hp(&mut self, hp: f32) {
        self.realProps.hp -= hp;
        self.basicProps.hp -= hp;
    }
    pub fn add_hp(&mut self, hp: f32) {
        self.realProps.hp += hp;
        self.basicProps.hp += hp;
    }

    pub fn check_death(&self) -> bool {
        self.realProps.hp <= 0.0
    }
}

static ItemPropsCounter: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Component)]
pub struct ItemProps {
    pub id: usize,
    pub realProps: RealProps,
    pub sender: Sender<ItemPropsSendData>,
    pub receiver: Receiver<ItemPropsSendData>,
}

impl ItemProps {
    pub fn new(basicProps: BasicProps) -> ItemProps {
        let (sender, receiver) = bounded(1);
        let itemProps = ItemProps {
            id: ItemPropsCounter.fetch_add(1, Ordering::Relaxed),
            realProps: RealProps::from(basicProps),
            sender,
            receiver,
        };
        itemProps
    }

    pub fn update_props(&self) {
        self.sender.send(ItemPropsSendData { id: self.id, realProps: self.realProps.clone() }).unwrap();
    }
}
