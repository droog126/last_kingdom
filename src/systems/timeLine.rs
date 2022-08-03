use super::debug::egui::DebugTable;
use bevy::{prelude::*, time::FixedTimestep};
use bevy_derive::{Deref, DerefMut};
pub struct TimeLinePlugin;

#[derive(Deref, DerefMut)]
pub struct TimeLine(pub i32);

impl Plugin for TimeLinePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup)
            .add_system_set(SystemSet::new().with_system(step).with_run_criteria(FixedTimestep::step(1.0 / 60.0)));
    }
}
fn startup(mut commands: Commands) {
    commands.insert_resource(TimeLine(0));
}
fn step(mut timeLine: ResMut<TimeLine>, mut debugTable: ResMut<DebugTable>) {
    timeLine.0 += 1;

    #[cfg(debug_assertions)]
    {
        debugTable.timeLine = Some(timeLine.0);
    }
}
