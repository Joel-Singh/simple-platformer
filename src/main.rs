use bevy::prelude::*;

#[derive(Component)]
struct SnakeHead;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,  setup)
        .add_systems(FixedUpdate, move_lilguy)
        .run();
}

const SNAKEHEAD_COLOR: Color =  Color::srgb(1.0, 0.0, 0.0);
const SNAKEHEAD_SIZE: Vec2 = Vec2::new(20.0, 20.0);

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
        SnakeHead
    ));
}

fn move_lilguy(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<SnakeHead>>,
) {
    let mut lilguy_transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        lilguy_transform.translation.x -= 10.0
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        lilguy_transform.translation.x += 10.0
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        lilguy_transform.translation.y += 10.0
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        lilguy_transform.translation.y -= 10.0
    }
}
