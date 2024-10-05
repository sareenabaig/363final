use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, spawn_camera);
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode
}

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        ImageBundle {
            transform: Transform::from_translation(Vec3::new(-300., 0., 0.)),
            background_color: BackgroundColor(Color::WHITE),
            style: Style {
                width: Val::Px(10.),
                height: Val::Px(150.),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        },
    ));
}

