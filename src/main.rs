mod background;
mod player;
mod ui;
use crate::background::*;
use crate::player::*;
use crate::ui::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(Startup, (setup_player, setup_background, setup_ui));
        app.add_systems(
            Update,
            (update_timer, move_player, detect_dig_collision).chain(),
        );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
