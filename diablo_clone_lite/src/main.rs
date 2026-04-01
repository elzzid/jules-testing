use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Town,
    Dungeon,
}

#[derive(Component)]
struct Player {
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Diablo Clone Lite".into(),
                resolution: (800_u32, 600_u32).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, (setup_camera, setup_player))
        .add_systems(Update, player_movement)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0), // Blue square
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player { speed: 300.0 },
    ));
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let mut direction = Vec2::ZERO;

    // Keyboard input (WASD)
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    // Gamepad input
    for gamepad in &gamepads {
        let left_stick = gamepad.left_stick();
        let deadzone = 0.1;

        if left_stick.length() > deadzone {
            // Remove deadzone from length and normalize
            let length = (left_stick.length() - deadzone) / (1.0 - deadzone);
            direction += left_stick.normalize() * length;
        }

        let dpad = gamepad.dpad();
        if dpad.length_squared() > 0.0 {
             direction += dpad;
        }
    }

    if direction.length_squared() > 0.0 {
        // Normalize to prevent faster diagonal movement when using keyboard
        // Gamepad direction might already be < 1.0, so we only normalize if length > 1.0
        let direction = direction.clamp_length_max(1.0);

        for (player, mut transform) in &mut query {
            transform.translation.x += direction.x * player.speed * time.delta_secs();
            transform.translation.y += direction.y * player.speed * time.delta_secs();
        }
    }
}
