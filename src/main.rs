use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct SnakeBody;

#[derive(Component)]
struct SnakeHead {
    body: Vec<Entity>
}

#[derive(Event)]
struct FruitEaten;

#[derive(Resource)]
struct MoveCooldown(Timer);

#[derive(Resource)]
struct FruitSpawnTimer(Timer);

#[derive(Component)]
struct Fruit;

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

fn tick_move_cooldown(
    time: Res<Time>,
    mut move_cooldown: ResMut<MoveCooldown>
) {
    move_cooldown.0.tick(time.delta());
}

fn ready_to_move(
    move_cooldown: Res<MoveCooldown>
) -> bool {
    move_cooldown.0.finished()
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            bevy::log::LogPlugin {
                level: bevy::log::Level::DEBUG,
                ..default()
            }
        ))
        .add_systems(Startup,  setup)
        .add_systems(FixedUpdate,
            (
                tick_move_cooldown,
                change_direction_on_input,
                move_snake.run_if(ready_to_move),
                spawn_fruits,
                map_position_to_transform,
                remove_snake_if_off_screen,
                handle_snake_fruit_collisions,
                add_snake_body_on_fruit_eaten
            ).chain()
        )
        .add_event::<FruitEaten>()
        .insert_resource(
            FruitSpawnTimer(
                Timer::from_seconds(FRUIT_SPAWN_COOLDOWN, TimerMode::Repeating)
            )
        )
        .insert_resource(
            MoveCooldown(Timer::from_seconds(MOVE_TIME, TimerMode::Repeating)),
        )
        .run();
}

const SNAKEHEAD_COLOR: Color =  Color::srgb(1.0, 1.0, 1.0);
const SPRITE_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const MOVE_TIME: f32 = 0.135;

const FRUIT_COLOR: Color = Color::linear_rgb(1.0, 0.0, 0.0);

const ARENA_WIDTH: i32 = 20;
const ARENA_HEIGHT: i32 = 20;

const ARENA_BEGINNING: i32 = -ARENA_WIDTH/2;
const ARENA_END: i32 = ARENA_WIDTH/2;

const ARENA_TOP: i32 = ARENA_HEIGHT/2;
const ARENA_BOTTOM: i32 = -ARENA_HEIGHT/2;

const FRUIT_SPAWN_COOLDOWN: f32 =  2.0;

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: SPRITE_SIZE.extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: SNAKEHEAD_COLOR,
                ..default()
            },
            ..default()
        },
        SnakeHead {
            body: vec![]
        },
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
    mut query: Query<(&mut Position, &mut Direction)>,
) {
    for (mut position, mut direction) in query.iter_mut() {
        match direction.as_mut() {
            Direction::Right => position.x += 1,
            Direction::Left => position.x -= 1,
            Direction::Up => position.y += 1,
            Direction::Down => position.y -= 1,
        };
    }
}

fn change_direction_on_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Direction, With<SnakeHead>>,
) {
    for mut snake_direction in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            *snake_direction = Direction::Left;
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            *snake_direction = Direction::Right;
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            *snake_direction = Direction::Up;
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            *snake_direction = Direction::Down;
        }
    }
}

fn handle_snake_fruit_collisions(
    snake_pos: Query<&Position, With<SnakeHead>>,
    fruits_pos: Query<(&Position, Entity), With<Fruit>>,
    mut ev_fruit_eaten: EventWriter<FruitEaten>,
    mut commands: Commands
) {
    let snake_pos = snake_pos.get_single();
    for (fruits_pos, fruit) in fruits_pos.iter() {
        if fruits_pos.x == snake_pos.as_ref().unwrap().x && fruits_pos.y == snake_pos.as_ref().unwrap().y {
            commands.entity(fruit).despawn();
            ev_fruit_eaten.send(FruitEaten);
        }
    }
}

fn add_snake_body_on_fruit_eaten (
    mut ev_fruit_eaten: EventReader<FruitEaten>,
    mut commands: Commands,
    mut snake_head: Query<&mut SnakeHead>
) {
    for _ in ev_fruit_eaten.read() {
        let mut snakehead = snake_head.get_single_mut().unwrap();
        let spawned_body = commands.spawn((
            SpriteBundle {
                transform: Transform {
                    scale: SPRITE_SIZE.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: SNAKEHEAD_COLOR,
                    ..default()
                },
                ..default()
            },
            SnakeBody,
            Direction::Up,
            Position {x: 0, y: 0}
        ));

        snakehead.body.push(spawned_body.id());
    }
}

fn remove_snake_if_off_screen(
    query: Query<(&Position, Entity)>,
    mut commands: Commands
) {
    for (position, entity) in query.iter() {
        let outside_bounds = 
            position.x < ARENA_BEGINNING ||
            position.x > ARENA_END ||
            position.y > ARENA_END ||
            position.y < ARENA_BEGINNING;
        if outside_bounds {
            commands.entity(entity).despawn()
        }
    }
}

fn spawn_fruits(
    mut commands: Commands,
    mut fruit_timer: ResMut<FruitSpawnTimer>,
    time: Res<Time>
) {
    fn random_position() -> Position {
        let mut rng = rand::thread_rng();
        return Position {
            x: rng.gen_range(ARENA_BEGINNING..ARENA_END + 1),
            y: rng.gen_range(ARENA_BOTTOM..ARENA_TOP + 1)
        }
    }

    if fruit_timer.0.tick(time.delta()).finished() {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    scale: SPRITE_SIZE.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: FRUIT_COLOR,
                    ..default()
                },
                ..default()
            },
            Fruit,
            random_position()
        ));
    }
}
