use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct TimerDisplay;

#[derive(Resource)]
pub struct GameTimer(pub Timer);

impl GameTimer {
    fn just_finished(&mut self, delta: Duration) -> bool {
        self.0.tick(delta).just_finished()
    }
}

pub fn setup_ui(mut commands: Commands) {
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
            parent.spawn((Text::new("Level 1"), TimerDisplay));
        });
}

pub fn update_timer(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    query: Single<&mut Text, With<TimerDisplay>>,
) {
    let mut text = query.into_inner();

    if timer.just_finished(time.delta()) {
        text.0 = format!("{}", time.elapsed().as_secs());
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Startup, setup_ui)
            .add_systems(Update, update_timer);
    }
}
