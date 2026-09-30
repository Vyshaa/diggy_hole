use bevy::prelude::*;

#[derive(Component)]
struct Cat;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_cat);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

fn add_cat(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Person, Name("Sophie".to_string())));
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
    ));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut query: Query<&mut Sprite, With<Cat>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut sprite in &mut query {
            sprite.flip_x ^= true;
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Sophie" {
            name.0 = "Sophie Grosse".to_string();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
