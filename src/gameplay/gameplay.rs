use bevy::prelude::*;
use rand::Rng;
use crate::components::{Dude, Score, SpawnTimer};

pub fn spawn_circle(
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let circle_mesh = meshes.add(Circle::new(25.0));

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-600.0..600.0);
        let y = rng.gen_range(-300.0..300.0);
        let r = rng.gen_range(0.0..1.0);
        let g = rng.gen_range(0.0..1.0);
        let b = rng.gen_range(0.0..1.0);

        commands.spawn((
            Dude,
            Mesh2d(circle_mesh),
            MeshMaterial2d(materials.add(Color::srgb(r, g, b))),
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}

pub fn despawn_on_click(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Dude>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut score: ResMut<Score>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.single() else { return };

        if let Some(cursor_pos) = window.cursor_position() {
            let Ok((camera, camera_transform)) = camera_query.single() else { return };

            if let Ok(click_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                for (entity, transform) in &query {
                    let circle_pos = transform.translation.truncate();
                    let distance = circle_pos.distance(click_pos);

                    if distance <= 25.0 {
                        commands.entity(entity).despawn();
                        score.0 += 1;
                        break;
                    }
                }
            }
        }
    }
}
