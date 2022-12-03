// 第四阶段，说全局唯一的一些数据，使用Resources
pub mod test_sprite_sheet;

use crate::components::test_comp::{Name, Person};
use bevy::prelude::*;

// 不知道里面怎么写的，对应函数参数的不同
fn greet_people(time: Res<Time>, query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}
