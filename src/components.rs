use bevy::prelude::*;

#[derive(Component)]
pub struct Dude;

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);
