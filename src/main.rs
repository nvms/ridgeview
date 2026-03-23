mod camera;
mod state;
mod terrain;
mod ui;

use bevy::prelude::*;

use camera::CameraPlugin;
use state::{TerrainParams, Technique};
use terrain::mesh::build_terrain_mesh;
use ui::UiPlugin;

#[derive(Component)]
struct TerrainEntity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ridgeview".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CameraPlugin)
        .add_plugins(UiPlugin)
        .init_resource::<TerrainParams>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, regenerate_terrain)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
    });
}

fn regenerate_terrain(
    mut commands: Commands,
    mut params: ResMut<TerrainParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<TerrainEntity>>,
) {
    if !params.dirty {
        return;
    }
    params.dirty = false;

    for entity in &query {
        commands.entity(entity).despawn();
    }

    let heightmap = match params.technique {
        Technique::Gradient => {
            terrain::gradient::generate(params.seed, params.octaves, params.gradient_falloff)
        }
        Technique::Dla => {
            terrain::dla::generate(params.seed, params.dla_walkers, params.blur_passes)
        }
    };

    let mesh = build_terrain_mesh(&heightmap);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        })),
        TerrainEntity,
    ));
}
