use bevy::{pbr::extract_lights, prelude::*, audio};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    println!("execute a example: ball_game");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Rolling Game".into(),
                        ..default()
                    },
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Hash, Debug)]
enum Action {
    Move,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D Camera
    commands.spawn(Camera2dBundle::default());

    // Spawn Player
    spawn_player(0, Vec2::new(-150.0, 0.0), &mut commands, &asset_server);
    spawn_player(1, Vec2::new(150.0, 0.0), &mut commands, &asset_server);
    // Spawn a triangle
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(150.0, 200.0, 0.0),
            texture: asset_server.load("textures/block_corner.png"),
            ..default()
        })
        .insert(Collider::triangle(
            Vec2::new(-32.0, 32.0),
            Vec2::new(32.0, -32.0),
            Vec2::new(-32.0, -32.0),
        ))
        .insert(RigidBody::Fixed)
        .insert(Restitution::coefficient(1.0));
}

fn spawn_player(
    id: usize,
    location: Vec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let image = if id == 0 {
        "textures/ball_blue_large.png"
    } else {
        "textures/ball_red_large.png"
    };
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(location.extend(1.0)),
            texture: asset_server.load(image),
            ..default()
        })
        .insert(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .insert(
                    if id == 0 {
                        VirtualDPad::wasd()
                    } else {
                        VirtualDPad::arrow_keys()
                    },
                    Action::Move,
                )
                .set_gamepad(Gamepad { id })
                .build(),
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(32.0))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(Damping {
            linear_damping: 0.6,
            angular_damping: 5.0,
        })
        .insert(Restitution::coefficient(1.0))
        .insert(Player);
}

const MOVE_FORCE: f32 = 1500.0;
fn movement(
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
    time: Res<Time>,
) {
    for (action_state, mut external_force) in &mut query {
        let axis_vector = action_state.clamped_axis_pair(Action::Move).unwrap().xy();
        external_force.force = axis_vector * MOVE_FORCE * time.delta_seconds();
    }
}


fn collision_sounds (
    rapier_context: Res<RapierContext>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>
){
    let mut just_collied = false;
    for pair in rapier_context.contact_pairs(){
        if pair.has_any_active_contacts() {
            just_collied = true;
        }
    }
    if just_collied {
        let sound = asset_server.load("impact/impactGlass_heavy_002.ogg");
        audio.play(sound);
    }
}