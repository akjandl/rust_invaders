#![allow(unused)] // silence unused warnings while learning

use bevy::{asset::Asset, prelude::*};

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const TIME_STEP: f32 = 1.0 / 60.0;

// Entity, Component, System, Resource

// START Resources
pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser: Handle<ColorMaterial>,
}
struct WinSize {
    w: f32,
    h: f32,
}
// END Resources

// START Components
struct Player;
struct PlayerReadyFire(bool);
struct Laser;
struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(500.0)
    }
}
// END Components

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders!".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system())
        .add_system(player_fire.system())
        .add_system(laser_movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let mut window = windows.get_primary_mut().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create main resources
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        laser: materials.add(asset_server.load(LASER_SPRITE).into()),
    });
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    // position window
    window.set_position(IVec2::new(1900, 0));
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<WinSize>) {
    // spawn a sprite
    let bottom = -win_size.h / 2.0;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_materials.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, bottom + 75.0 / 4.0 + 5.0, 10.0),
                scale: Vec3::new(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerReadyFire(true))
        .insert(Speed::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform), With<Player>>,
) {
    if let Ok((speed, mut transform)) = query.single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::A) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::D) {
            1.0
        } else {
            0.0
        };
        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}

fn player_fire(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut query: Query<(&Transform, &mut PlayerReadyFire), With<Player>>,
) {
    if let Ok((player_tf, mut ready_fire)) = query.single_mut() {
        if ready_fire.0 && kb.pressed(KeyCode::Space) {
            let x = player_tf.translation.x;
            let y = player_tf.translation.y;
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y + 15.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(Speed::default());
            ready_fire.0 = false;
        }

        if kb.just_released(KeyCode::Space) {
            ready_fire.0 = true;
        }
    }
}

fn laser_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform), With<Laser>>,
) {
    for (laser_entity, speed, mut laser_tf) in query.iter_mut() {
        let translation = &mut laser_tf.translation;
        translation.y += speed.0 * TIME_STEP;
        if translation.y > win_size.h {
            commands.entity(laser_entity).despawn();
        }
    }
}
