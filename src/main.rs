mod components;
mod ui;
mod gameplay;

use bevy::prelude::*;
use ui::UiPlugin;
use gameplay::GameplayPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((UiPlugin, GameplayPlugin))
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
