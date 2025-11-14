mod ui;

use bevy::prelude::*;
pub use ui::{setup_score_text, update_score_text};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_score_text);
        app.add_systems(Update, update_score_text);
    }
}
