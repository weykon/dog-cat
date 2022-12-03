use crate::components::test_comp::{Name, Person};
use bevy::prelude::*;

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

pub fn hello_world() {
    println!("hello bevy");

    println!("this is loading at game runtime starting ...");

    let may_I_read_something_from_disk = String::from("OK");

    println!(
        "may_I_read_something_from_disk: {}",
        may_I_read_something_from_disk
    );

    println!(
        "may_I_read_something_from_disk\nmay I use again?: {}",
        may_I_read_something_from_disk
    );
}

// 这个将会放在 startup_system上运行
pub fn bundle_some_systems_fn(mut commands: Commands) {
    // 创建三个
    commands.spawn((Person, Name("Weykon".to_string())));
    commands.spawn((Person, Name("ABC".to_string())));
    commands.spawn((Person, Name("CCC".to_string())));
}

// 第二阶段
pub fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}", name.0);
    }
}
// 这里参数，迭代所有含有Person组件的实体下的Name，
// 意为： 拿 &Name 并有着 Person comp