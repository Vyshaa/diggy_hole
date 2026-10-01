use std::time::Duration;

use bevy::{prelude::*, window::WindowResized};

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Ui;

#[derive(Component)]
struct Cat;

#[derive(Resource)]
struct GameTimer(Timer);

impl GameTimer {
    fn just_finished(&mut self, delta: Duration) -> bool {
        self.0.tick(delta).just_finished()
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(Startup, set_scene);
        app.add_systems(Update, (reposition_ui, (update_timer, move_cat).chain()));
    }
}

fn set_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, MainCamera));

    let cat = asset_server.load("cat.png");
    commands.spawn((
        Cat,
        Sprite {
            image: cat.clone(),
            custom_size: Some(Vec2::new(100., 225.)),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.),
    ));

    commands.spawn((Ui, Text2d::new("0"), Transform::from_xyz(0., 0., 0.)));
}

fn move_cat(time: Res<Time>, mut query: Query<(&mut Sprite, &mut Transform), With<Cat>>) {
    for (mut sprite, mut transform) in &mut query {
        if sprite.flip_x {
            transform.translation.x += 150. * time.delta_secs();
        } else {
            transform.translation.x -= 150. * time.delta_secs();
        }
        if transform.translation.x > 200. {
            sprite.flip_x = false;
        } else if transform.translation.x < -200. {
            sprite.flip_x = true;
        }
    }
}

fn reposition_ui(
    mut query: Query<&mut Transform, With<Ui>>,
    mut resize_events: MessageReader<WindowResized>,
) {
    let Some(event) = resize_events.read().last() else {
        return;
    };
    for mut pos in &mut query {
        pos.translation.x = -event.width / 2. + 10.;
        pos.translation.y = event.height / 2. - 10.;
    }
}

fn update_timer(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut query: Query<&mut Text2d, With<Ui>>,
) {
    for mut text in &mut query {
        if timer.just_finished(time.delta()) {
            text.0 = format!("{}", time.elapsed().as_secs());
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
