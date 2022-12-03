use bevy::prelude::*;

mod components;
mod entities;
mod plugins;
mod resources;
mod systems;
fn main() {
    // App::new()
    // .add_plugins(DefaultPlugins)
    // 这里有 plugins, plugin, 一个是加载plugin group,另一个就单个
    // .add_plugin(plugins::hello_plugin::HelloPlugin)
    // .add_startup_system(systems::test_system::bundle_some_systems_fn)
    // .add_system(systems::test_system::hello_world)
    // .add_system(systems::test_system::greet_people)
    // 上面注释的是在hello_plugin做了一样的实现
    // .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    // .add_startup_system(resources::test_sprite_sheet::setup_res_load)
    // .add_system(systems::dog_anim::animate_sprite)
    // .run();

    // 以下是 https://www.youtube.com/watch?v=fFm1IxuPQxA 的演练
}
