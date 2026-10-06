use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct Velocity(f32);

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
            velocity: Velocity(100.),
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

#[derive(Component)]
struct HealthDisplay;

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
        app.add_systems(
            Update,
            (update_timer, update_health_bar, move_player).chain(),
        );
    }
}

fn set_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, MainCamera));
    commands.spawn(PlayerBundle::new(&asset_server));

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(10.)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((Text::new("0"), TimerDisplay));
            parent.spawn((Text::new("0"), HealthDisplay));
        });
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut Transform, &Velocity), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut sprite, mut transform, velocity) in &mut query {
        if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            transform.translation.x += velocity.0 * time.delta_secs();
            sprite.flip_x = true;
        } else if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            transform.translation.x -= velocity.0 * time.delta_secs();
            sprite.flip_x = false;
        }

        if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            transform.translation.y += velocity.0 * time.delta_secs();
        } else if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            transform.translation.y -= velocity.0 * time.delta_secs();
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

fn update_health_bar(
    mut ui_query: Query<&mut Text, With<HealthDisplay>>,
    health_query: Query<&Health, With<Player>>,
) {
    let mut text = ui_query.single_mut().unwrap();
    text.0 = health_query.single().unwrap().0.to_string();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
