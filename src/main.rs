use bevy::{prelude::*};

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,  setup)
        .add_systems(FixedUpdate, change_direction_snakehead)
        .add_systems(FixedUpdate, map_position_to_transform)
        .run();
}

const SNAKEHEAD_COLOR: Color =  Color::srgb(1.0, 0.0, 0.0);
const SNAKEHEAD_SIZE: Vec2 = Vec2::new(20.0, 20.0);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: SNAKEHEAD_SIZE.extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: SNAKEHEAD_COLOR,
                ..default()
            },
            ..default()
        },
        SnakeHead,
        Position {
            x: 3,
            y: 3
        },
        Direction::Up
    ));
}

fn map_position_to_transform(
    windows: Query<&Window>,
    mut query: Query<(&mut Position, &mut Transform)>
) {
    let window = windows.single();
    for (position, mut transform) in &mut query {
        let x_increment = window.width() / ARENA_WIDTH as f32;
        transform.translation.x = position.x as f32*x_increment;

        let y_increment = window.width() / ARENA_HEIGHT as f32;
        transform.translation.y = position.y as f32*y_increment;
    }
}

fn change_direction_snakehead(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Direction, With<SnakeHead>>,
) {
    let mut snake_direction = query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        *snake_direction = Direction::Left;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        *snake_direction = Direction::Right;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        *snake_direction = Direction::Up;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        *snake_direction = Direction::Down;
    }
}
