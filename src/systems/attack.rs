use bevy::prelude::*;

use super::collision::CollisionResultArr;
// pub struct AttackPlugin;
// impl Plugin for AttackPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_startup_system(startup).add_system(step)
//     }
// }
#[derive(Component)]
pub struct AttackBoxTag;

#[derive(Component, Debug)]
pub struct AttackStorehouseArr {
    pub arr: Vec<AttackEvent>,
}

#[derive(Debug, Component, Clone)]
pub struct AttackEvent {
    pub id: Entity,
    pub damage: f32,
    pub nextTime: i32,

    pub repelData: Option<RepelData>,
}
pub struct AttackEventPart {
    pub damage: f32,
    pub nextTime: i32,

    pub repelData: Option<RepelData>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct RepelData {
    pub dif: Vec3,
    pub timeLen: i32,
}

pub fn attack_event_distribution_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CollisionResultArr, &AttackEvent), With<AttackBoxTag>>,
    mut attackQuery: Query<&mut AttackStorehouseArr, Without<AttackBoxTag>>,
) {
    for (entity, mut collisionResultArr, attackEvent) in query.iter_mut() {
        // println!("lookMe {:?}", collisionResultArr);

        for item in collisionResultArr.arr.iter() {
            if let Ok(mut attackStorehouseArr) = attackQuery.get_mut(item.id) {
                attackStorehouseArr.arr.push(attackEvent.clone());
            }
        }

        collisionResultArr.arr.clear();
        commands.entity(entity).despawn();
    }
}
