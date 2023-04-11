use bevy::{prelude::*, render::view::window, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .run();
}

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Enemy {}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
        ..default()
    });
}

fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let nof_enemies = 4;

    for _ in (0..nof_enemies) {
        let x: f32 = rand::random();
        let y: f32 = rand::random();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(window.width() * x, window.height() * y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {},
        ));
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
            direction.x -= 1.0;
        }

        if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
            direction.x += 1.0;
        }

        if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
            direction.y += 1.0;
        }

        if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
            direction.y -= 1.0;
        }

        direction = direction.normalize_or_zero();

        let player_speed = 500.0;

        transform.translation += direction * player_speed * time.delta_seconds();
    }
}

fn confine_player_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut t) = player_query.get_single_mut() {
        t.translation.x = t.translation.x.clamp(0.0, window.width());
        t.translation.y = t.translation.y.clamp(0.0, window.height());
    }
}
