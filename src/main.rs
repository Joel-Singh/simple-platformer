use bevy::prelude::*;

#[derive(Component)]
struct LilGuy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,  setup)
        .run();
}

const LILGUY_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const LILGUY_COLOR: Color =  Color::srgb(1.0, 0.0, 0.0);

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
