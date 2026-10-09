pub mod background;
pub mod enemy;
pub mod player;
pub mod ui;

pub use self::game::GamePlugin;

mod game {
    use crate::{
        background::BackgroundPlugin, enemy::EnemyPlugin, player::PlayerPlugin, ui::UiPlugin,
    };
    use bevy::prelude::*;

    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((PlayerPlugin, EnemyPlugin, BackgroundPlugin, UiPlugin));
        }
    }
}
