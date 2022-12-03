use crate::entities::anim::*;
use bevy::prelude::*;
pub fn setup_res_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/doggy3.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(25.0, 19.0),
        5,
        5,
        Some(Vec2::new(0.0, 0.0)),
        Some(Vec2::new(0.0, 0.0)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
}
