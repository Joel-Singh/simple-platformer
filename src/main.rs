use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct SnakeBody;

#[derive(Component)]
struct SnakeHead;

#[derive(Resource)]
struct SnakeBodyVec(Vec<Entity>);

#[derive(Bundle)]
struct SnakeSpriteBundle {
    sprite_bundle: SpriteBundle    
}

impl Default for SnakeSpriteBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
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
        }
    }
}

#[derive(Event)]
struct FruitEaten;

#[derive(Resource)]
struct MoveCooldown(Timer);

#[derive(Resource)]
struct FruitSpawnTimer(Timer);

#[derive(Component)]
struct Fruit;

#[derive(Component, Clone, Copy)]
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

fn create_position(x: i32, y: i32) -> Position {
    Position { x, y }
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
                move_snake_bodies.run_if(ready_to_move),
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
        .insert_resource(
           SnakeBodyVec(Vec::new())
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
    mut body_vec: ResMut<SnakeBodyVec>
) {
    commands.spawn(Camera2dBundle::default());
    
    let snake_head = commands.spawn((
        SnakeSpriteBundle::default(),
        SnakeHead,
        create_position(0, 0),
        Direction::Up
    ));

    body_vec.0.push(snake_head.id())
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

fn move_snakehead(
    mut query: Query<(&mut Position, &Direction), With<SnakeHead>>,
) {
    let (mut position, direction) = query.get_single_mut().unwrap();
    *position = position_infront(&direction, &position)
}

fn move_snake_bodies(
    mut query_bodies: Query<(&mut Direction, &mut Position)>,
) {
    for (direction, mut position) in query_bodies.iter_mut() {
        *position = position_infront(&direction, &position);
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

fn change_snake_body_direction(
    mut body_direction_query: Query<&mut Direction, With<SnakeBody>>,
    mut head_direction_query: Query<&mut Direction, (With<SnakeHead>, Without<SnakeBody>)>,
) {
    for mut body_direction in body_direction_query.iter_mut() {
        let head_direction = *head_direction_query.get_single_mut().unwrap();
        *body_direction = head_direction;
    }
}

fn add_snake_body_on_fruit_eaten (
    mut ev_fruit_eaten: EventReader<FruitEaten>,
    mut commands: Commands,
    body_query: Query<(&Direction, &Position)>,
    mut body_vec: ResMut<SnakeBodyVec>
) {
    for _ in ev_fruit_eaten.read() {
        let last_entity = *body_vec.0.last().unwrap();
        let (tail_direction, tail_position) = body_query.get(last_entity).unwrap();
        let spawned_body = commands.spawn((
            SnakeSpriteBundle::default(),
            SnakeBody,
            *tail_direction,
            position_behind(tail_direction, tail_position),
        ));

        body_vec.0.push(spawned_body.id());
    }
}

fn position_behind(direction: &Direction, position: &Position) -> Position {
    match direction {
        Direction::Up => create_position(position.x, position.y - 1),
        Direction::Down => create_position(position.x, position.y + 1 ),
        Direction::Left => create_position(position.x + 1, position.y ),
        Direction::Right => create_position(position.x - 1, position.y )
    }
}

fn position_infront(direction: &Direction, position: &Position) -> Position {
    match direction {
        Direction::Up => create_position(position.x, position.y + 1),
        Direction::Down => create_position(position.x, position.y - 1 ),
        Direction::Left => create_position(position.x - 1, position.y ),
        Direction::Right => create_position(position.x + 1, position.y )
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
        Position {
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
