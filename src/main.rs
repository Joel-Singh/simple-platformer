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
) {
    let mut lilguy_transform = query.single_mut();
    let mut x_direction = 0.0;

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
