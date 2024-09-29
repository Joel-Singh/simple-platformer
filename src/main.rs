use bevy::prelude::*;

#[derive(Component)]
struct LilGuy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,  setup)
        .add_systems(FixedUpdate, move_lilguy)
        .run();
}

const LILGUY_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const LILGUY_COLOR: Color =  Color::srgb(1.0, 0.0, 0.0);

const FLOOR_HEIGHT: f32 = 0.0;
const LILGUY_SPEED: f32 = 500.0;

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: LILGUY_SIZE.extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: LILGUY_COLOR,
                ..default()
            },
            ..default()
        },
        LilGuy
    ));
}

fn move_lilguy(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<LilGuy>>,
    time: Res<Time>,
) {
    let mut lilguy_transform = query.single_mut();
    let mut x_direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        x_direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        x_direction += 1.0;
    }

    let mut lilguy_transform = query.single_mut();
    let mut y_direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        y_direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        y_direction += 1.0;
    }


    // Calculate the new horizontal paddle position based on player input
    let new_position =
        lilguy_transform.translation.x + x_direction * LILGUY_SPEED * time.delta_seconds();

    lilguy_transform.translation.x = new_position;
    lilguy_transform.translation.y = lilguy_transform.translation.y.max(FLOOR_HEIGHT);
}
