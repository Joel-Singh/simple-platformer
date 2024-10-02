use bevy::prelude::*;

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,  setup)
        .add_systems(FixedUpdate, move_lilguy)
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
        }
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

        let y_increment = window.width() / ARENA_WIDTH as f32;
        transform.translation.y = position.y as f32*y_increment;
    }
}

fn move_lilguy(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Position, With<SnakeHead>>,
) {
    let mut lilguy_position = query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        lilguy_position.x -= 1;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        lilguy_position.x += 1;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        lilguy_position.y += 1;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        lilguy_position.y -= 1;
    }
}
