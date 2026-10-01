use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    health: Health,
    velocity: Velocity,
    sprite: Sprite,
    transform: Transform,
}

impl PlayerBundle {
    fn new(asset_server: &AssetServer) -> Self {
        let cat = asset_server.load("cat.png");
        Self {
            player: Player,
            health: Health(100),
            velocity: Velocity(Vec2::ZERO),
            sprite: Sprite {
                image: cat.clone(),
                custom_size: Some(Vec2::new(100., 225.)),
                image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
        }
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct TimerDisplay;

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
        app.add_systems(Update, (update_timer, move_player).chain());
    }
}

fn set_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, MainCamera));
    commands.spawn(PlayerBundle::new(&asset_server));

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.),
                    left: Val::Px(10.),
                    ..Default::default()
                },
                Text::new("0"),
                TimerDisplay,
            ));
        });
}

fn move_player(time: Res<Time>, mut query: Query<(&mut Sprite, &mut Transform), With<Player>>) {
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

fn update_timer(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut query: Query<&mut Text, With<TimerDisplay>>,
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
