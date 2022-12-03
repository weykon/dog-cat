// 为了更好的组织代码
// for better organization
// move all "hello" logic to a plugin

use crate::{components::test_comp::GreetTimer, systems};
use bevy::prelude::*;
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(systems::test_system::bundle_some_systems_fn)
            .add_system(systems::test_system::hello_world)
            .add_system(systems::test_system::greet_people);
    }
}
