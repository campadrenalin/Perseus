use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

// https://github.com/rust-analyzer/rust-analyzer/issues/8654
fn vec2(x: f32, y: f32) -> Vector2<f32> { [x, y].into() }

const RAPIER_SCALE:f32 = 20.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Player Movement Example".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera.system())
        .add_startup_system(spawn_players.system())
        .add_startup_system(spawn_ball.system())
        .add_system(keyboard_movement.system())
        .add_system(print_positions.system())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}

#[derive(Debug)]
enum EntityPrimaryTag {
    Camera,
    Player,
    Ball,
    // Wall,
}

#[derive(Component)]
struct Tag(EntityPrimaryTag);

fn spawn_camera(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // Set config constants
    rapier_config.gravity = Vector2::zeros();
    rapier_config.scale = RAPIER_SCALE;

    // Actually spawn camera
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Tag(EntityPrimaryTag::Camera));
}

#[derive(Component)]
struct MoveSpeed(f32);

#[derive(Component)]
struct KeyboardMoveable {
    up: bevy::input::keyboard::KeyCode,
    down: bevy::input::keyboard::KeyCode,
}

fn spawn_players(mut commands: Commands) {
    let spacing = 20.0;
    spawn_player(&mut commands, -spacing, KeyboardMoveable {
        up: bevy::input::keyboard::KeyCode::W,
        down: bevy::input::keyboard::KeyCode::S,
    });
    spawn_player(&mut commands, spacing, KeyboardMoveable {
        up: bevy::input::keyboard::KeyCode::Up,
        down: bevy::input::keyboard::KeyCode::Down,
    });
}

fn spawn_player(
    commands: &mut Commands,
    x: f32,
    motion: KeyboardMoveable
) {
    let player_size_x = 2.0;
    let player_size_y = 20.0;

    // Spawn player
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0,0.0,0.0),
                custom_size: Some(Vec2::new(
                    player_size_x * RAPIER_SCALE,
                    player_size_y * RAPIER_SCALE
                )),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::KinematicVelocityBased.into(),
            position: [x, 0.0].into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(player_size_x/2.0, player_size_y/2.0).into(),
            material: ColliderMaterial {
                restitution: 1.0,
                ..Default::default()
            }.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(0))
        .insert(MoveSpeed(30.0))
        .insert(motion)
        .insert(Tag(EntityPrimaryTag::Player));
}

fn spawn_ball(mut commands: Commands) {
    let ball_size_x = 1.0;
    let ball_size_y = 1.0;

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(ball_size_x * RAPIER_SCALE, ball_size_y * RAPIER_SCALE)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            velocity: RigidBodyVelocity {
                linvel: Vec2::new(20.0, 1.0).into(),
                ..Default::default()
            }.into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            position: [0.0, 0.0].into(),
            shape: ColliderShape::cuboid(ball_size_x/2.0, ball_size_y/2.0).into(),
            material: ColliderMaterial {
                restitution: 1.0,
                ..Default::default()
            }.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1))
        .insert(MoveSpeed(100.0))
        .insert(Tag(EntityPrimaryTag::Ball));
}

fn keyboard_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(
        &MoveSpeed,
        &KeyboardMoveable,
        &mut RigidBodyVelocityComponent)>,
) {
    for (speed, keys, mut rb_vels) in player_info.iter_mut() {
        let up = keyboard_input.pressed(keys.up);
        let down = keyboard_input.pressed(keys.down);

        let x_axis = 0 as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = vec2(x_axis as f32, y_axis as f32);
        if move_delta != Vector2::zeros() {
            move_delta /= move_delta.magnitude();
        }

        rb_vels.linvel = move_delta * speed.0;
    }
}

fn print_positions(
    keyboard_input: Res<Input<KeyCode>>,
    entity_info: Query<(&Tag, &Transform)>,
) {
    if !keyboard_input.pressed(KeyCode::X) {
        return
    }
    println!(" === Printing positions: === ");

    for (tag, transform) in entity_info.iter() {
        println!("{:?} {}", tag.0, transform.translation);
    }
}