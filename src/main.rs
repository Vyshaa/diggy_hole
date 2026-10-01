use bevy::{prelude::*, window::WindowResized};

#[derive(Component)]
struct Cat;

#[derive(Resource)]
struct GameTimer(Timer);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_cat);
        app.add_systems(Update, (resize, (update_timer, move_cat).chain()));
    }
}

fn add_cat(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
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
    commands.spawn((Text2d::new("0"), Transform::from_xyz(0., 0., 0.)));
}

fn move_cat(time: Res<Time>, mut query: Query<(&mut Sprite, &mut Transform), With<Cat>>) {
    for (mut sprite, mut transform) in &mut query {
        match sprite.flip_x {
            true => transform.translation.x += 150. * time.delta_secs(),
            false => transform.translation.x -= 150. * time.delta_secs(),
        }
        if transform.translation.x > 200. {
            sprite.flip_x = false;
        } else if transform.translation.x < -200. {
            sprite.flip_x = true;
        }
    }
}

fn resize(mut query: Query<&mut Transform>, mut resize: MessageReader<WindowResized>) {
    for mut pos in &mut query {
        for e in resize.read() {
            pos.translation.x = -e.width / 2. + 10.;
            pos.translation.y = e.height / 2. - 10.;
        }
    }
}

fn update_timer(time: Res<Time>, mut timer: ResMut<GameTimer>, mut query: Query<&mut Text2d>) {
    for mut text in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
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
