use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

// https://github.com/rust-analyzer/rust-analyzer/issues/8654
fn vec2(x: f32, y: f32) -> Vector2<f32> { [x, y].into() }

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
        .add_startup_system(spawn_player.system())
        .add_system(player_movement.system())
        .add_system(print_positions.system())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}

#[derive(Debug)]
enum EntityPrimaryTag {
    Camera,
    Player,
}

#[derive(Component)]
struct Tag(EntityPrimaryTag);

fn spawn_camera(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // Set config constants
    rapier_config.gravity = Vector2::zeros();
    rapier_config.scale = 20.0;

    // Actually spawn camera
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Tag(EntityPrimaryTag::Camera));
}

#[derive(Component)]
struct Player(f32);

fn spawn_player(mut commands: Commands, rapier_config: ResMut<RapierConfiguration>) {
    let sprite_size_x = 40.0;
    let sprite_size_y = 40.0;

    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;

    // Spawn player
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0,0.0,0.0),
                custom_size: Some(Vec2::new(sprite_size_x, sprite_size_y)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle::default())
        .insert_bundle(ColliderBundle {
            position: [collider_size_x/2.0, collider_size_y/2.0].into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(0))
        .insert(Player(300.0))
        .insert(Tag(EntityPrimaryTag::Player));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut player_info: Query<(&Player, &mut RigidBodyVelocityComponent)>,
) {
    for (player, mut rb_vels) in player_info.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
        
        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = vec2(x_axis as f32, y_axis as f32);
        if move_delta != Vector2::zeros() {
            move_delta /= move_delta.magnitude() * rapier_parameters.scale;
        }

        rb_vels.linvel = move_delta * player.0;
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