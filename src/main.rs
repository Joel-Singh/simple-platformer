use bevy::{prelude::*};

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct MoveCooldown(Timer);

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
        .add_systems(FixedUpdate, move_snake)
        .run();
}

const SNAKEHEAD_COLOR: Color =  Color::srgb(1.0, 0.0, 0.0);
const SNAKEHEAD_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const MOVE_TIME: f32 = 0.01;

const ARENA_WIDTH: u32 = 100;
const ARENA_HEIGHT: u32 = 100;

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
        MoveCooldown(Timer::from_seconds(MOVE_TIME, TimerMode::Once)),
        Position { x: 0, y: 0 },
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

fn move_snake(
    time: Res<Time>,
    mut query: Query<(&mut MoveCooldown, &mut Position, &mut Direction)>,
) {
    let (mut cooldown, mut position, mut direction) = query.single_mut();
    if cooldown.0.tick(time.delta()).finished() {
        match direction.as_mut() {
            Direction::Right => position.x += 1,
            Direction::Left => position.x -= 1,
            Direction::Up => position.y += 1,
            Direction::Down => position.y -= 1,
        }
        cooldown.0.reset()
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
