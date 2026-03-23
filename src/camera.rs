use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, disable_camera_over_ui);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        PanOrbitCamera {
            radius: Some(200.0),
            pitch: Some(0.6),
            yaw: Some(0.8),
            target_focus: Vec3::new(0.0, 10.0, 0.0),
            ..default()
        },
    ));
}

fn disable_camera_over_ui(
    mut contexts: EguiContexts,
    mut camera: Query<&mut PanOrbitCamera>,
) {
    let ctx = contexts.ctx_mut();
    let over_ui = ctx.is_pointer_over_area();
    for mut cam in &mut camera {
        cam.enabled = !over_ui;
    }
}
