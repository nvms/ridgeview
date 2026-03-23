use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        PanOrbitCamera {
            radius: Some(200.0),
            pitch: Some(-0.6),
            yaw: Some(0.4),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 100.0, 200.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
