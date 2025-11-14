mod gameplay;

use bevy::prelude::*;
use crate::components::{Score, SpawnTimer};
pub use gameplay::{spawn_circle, despawn_on_click};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
        app.add_systems(Startup, crate::setup_camera);
        app.add_systems(Update, (spawn_circle, despawn_on_click));
    }
}
