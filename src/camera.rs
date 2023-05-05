use crate::actions::PlayerMovedEvent;
use bevy::prelude::*;

use crate::graphics::TILE_SIZE;

pub fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        1.0 * TILE_SIZE,
        1.0 * TILE_SIZE,
        camera.transform.translation.z,
    );
    commands.spawn(camera);
}

pub fn follow_player(
    mut q_camera: Query<&mut Transform, With<Camera>>,
    mut ev_pmov: EventReader<PlayerMovedEvent>,
) {
    for movement in ev_pmov.iter() {
        debug!("The player moved to {:?}", movement.0);

        let mut transform = q_camera.get_single_mut().unwrap();
        let translation = Vec3::new(
            movement.0.x as f32 * TILE_SIZE,
            movement.0.y as f32 * TILE_SIZE,
            transform.translation.z,
        );

        transform.translation = translation;
    }
}
