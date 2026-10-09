use bevy::prelude::*;

mod background;
mod player;
mod ui;

use crate::background::*;
use crate::player::*;
use crate::ui::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, BackgroundPlugin, UiPlugin));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
