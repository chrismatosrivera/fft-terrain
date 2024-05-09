//Christian Matos Rivera

//Terrain Generation with Several Algorithms

//The following file implements a set of heighmap generation functions for terrain generation based of several algorithms:
// - Midpoint Displacement & Diamond Square
// - Fast Fourier Transforms
// - Noise Generators

//It uses the Bevy engine to render the heightmaps with simple lighting and materials. Therefore, this
//file contains functions related to the bevy engine. Most of it is boiler plate from Bevy documentation.

//You may find terrain generators, utilities and noise generators in the files:
// - terrain.rs
// - noise.rs
// - fft_utils.rs
// - fractal_analysis.rs


use bevy::prelude::*;
use bevy::{
    pbr::{light_consts, CascadeShadowConfigBuilder},
};
use bevy::render::{
    mesh::{Indices, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
    camera::{Exposure, PhysicalCameraParameters},
};
use std::f32::consts::PI;

use crate::noise::noise_terrain;
use crate::noise::NoiseType;
use crate::fractal_analysis::calculate_fractal_dimension;

mod terrain;
mod fft_utils;
mod noise;
mod fractal_analysis;

// Define a "marker" component to mark the custom mesh. Marker components are often used in Bevy for
// filtering entities in queries with With, they're usually not queried directly since they don't contain information within them.
#[derive(Component)]
struct CustomUV;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {

    let n = 8;
    //let heightmap = midpoint_displacement(n, 0.75, 0.50, 1000.0);
    //let heightmap = fft_terrain(n);
    let noiseType = NoiseType::Perlin;
    let heightmap = noise_terrain(n, noiseType);
    let size = 2_usize.pow(n);

    let terrain_mesh_handle: Handle<Mesh> = meshes.add(create_mesh(&heightmap));

    // Render the mesh with the custom texture using a PbrBundle, add the marker.
    commands.spawn((
        PbrBundle {
            mesh: terrain_mesh_handle,
            material: materials.add(StandardMaterial {
                ..default()
            }),
            ..default()
        },
        CustomUV,
    ));

    // Transform for the camera based on size of the mesh.
    let camera_transform =
        Transform::from_xyz((size as f32) - 10.0, (size as f32) - 10.0, 22.0).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands.spawn(Camera3dBundle {
        transform: camera_transform,
        ..default()
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 20.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 20.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Text to describe the controls.
    commands.spawn(
        TextBundle::from_section(
            "Controls:\n X/Y/Z: Rotate\n M: Generate Midpoint Displacement \n F: Generate FFT \n P: Generate Perlin Noise \n S: Generate Simplex Noise \n W: Generate Worley Noise",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

// System to receive input from the user,
fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Transform, With<CustomUV>>,
    mut entity_query: Query<Entity, With<CustomUV>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let n = 8;
    if keyboard_input.pressed(KeyCode::KeyX) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_seconds() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyY) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_seconds() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_seconds() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        for mut transform in &mut query {
            transform.look_to(Vec3::NEG_Z, Vec3::Y);
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        for entity in entity_query.iter() {
            commands.entity(entity).despawn();
        }

        
        let heightmap = terrain::midpoint_displacement(n, 0.75, 0.50, 45.0);
        let size = 2_usize.pow(n);

        let terrain_mesh_handle: Handle<Mesh> = meshes.add(create_mesh(&heightmap));

        // Render the mesh with the custom texture using a PbrBundle, add the marker.
        commands.spawn((
            PbrBundle {
                mesh: terrain_mesh_handle,
                material: materials.add(StandardMaterial {
                    ..default()
                }),
                ..default()
            },
            CustomUV,
        ));
    }

    if keyboard_input.just_pressed(KeyCode::KeyF) {
        for entity in entity_query.iter() {
            commands.entity(entity).despawn();
        }

        let heightmap = terrain::fft_terrain(n);
        let size = 2_usize.pow(n);

        let terrain_mesh_handle: Handle<Mesh> = meshes.add(create_mesh(&heightmap));

        // Render the mesh with the custom texture using a PbrBundle, add the marker.
        commands.spawn((
            PbrBundle {
                mesh: terrain_mesh_handle,
                material: materials.add(StandardMaterial {
                    ..default()
                }),
                ..default()
            },
            CustomUV,
        ));
    }

    if keyboard_input.just_pressed(KeyCode::KeyP) {
        for entity in entity_query.iter() {
            commands.entity(entity).despawn();
        }

        let noiseType = noise::NoiseType::Perlin;
        let heightmap = noise::noise_terrain(n, noiseType);
        let size = 2_usize.pow(n);

        let terrain_mesh_handle: Handle<Mesh> = meshes.add(create_mesh(&heightmap));

        // Render the mesh with the custom texture using a PbrBundle, add the marker.
        commands.spawn((
            PbrBundle {
                mesh: terrain_mesh_handle,
                material: materials.add(StandardMaterial {
                    ..default()
                }),
                ..default()
            },
            CustomUV,
        ));
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        for entity in entity_query.iter() {
            commands.entity(entity).despawn();
        }

        let noiseType = noise::NoiseType::Worley;
        let heightmap = noise::noise_terrain(n, noiseType);
        let size = 2_usize.pow(n);

        let terrain_mesh_handle: Handle<Mesh> = meshes.add(create_mesh(&heightmap));

        // Render the mesh with the custom texture using a PbrBundle, add the marker.
        commands.spawn((
            PbrBundle {
                mesh: terrain_mesh_handle,
                material: materials.add(StandardMaterial {
                    ..default()
                }),
                ..default()
            },
            CustomUV,
        ));
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        for entity in entity_query.iter() {
            commands.entity(entity).despawn();
        }

        let noiseType = noise::NoiseType::Simplex;
        let heightmap = noise::noise_terrain(n, noiseType);
        let size = 2_usize.pow(n);

        let terrain_mesh_handle: Handle<Mesh> = meshes.add(create_mesh(&heightmap));

        // Render the mesh with the custom texture using a PbrBundle, add the marker.
        commands.spawn((
            PbrBundle {
                mesh: terrain_mesh_handle,
                material: materials.add(StandardMaterial {
                    ..default()
                }),
                ..default()
            },
            CustomUV,
        ));
    }
}

// Create a mesh that bevy can render using a heightmap
#[rustfmt::skip]
fn create_mesh(heightmap: &Vec<Vec<f32>>) -> Mesh {
   let width = heightmap[0].len();
    let height = heightmap.len();

    // Calculate the center of the mesh
    let center_x = width as f32 / 2.0;
    let center_y = 0.0; 
    let center_z = height as f32 / 2.0;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = vec![[0.0; 3]; width * height]; 
    let mut indices: Vec<u32> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let adjusted_x = x as f32 - center_x;
            let adjusted_y = heightmap[y][x];
            let adjusted_z = y as f32 - center_z;

            positions.push([adjusted_x, adjusted_y, adjusted_z]);
        }
    }

    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let i = y * width + x;
            let top_left = i;
            let top_right = i + 1;
            let bottom_left = i + width;
            let bottom_right = i + width + 1;

            indices.extend_from_slice(&[top_left as u32, bottom_left as u32, top_right as u32]);
            indices.extend_from_slice(&[top_right as u32, bottom_left as u32, bottom_right as u32]);

            // Calculate normals for the two triangles
            let tri_1 = [positions[top_left], positions[bottom_left], positions[top_right]];
            let tri_2 = [positions[top_right], positions[bottom_left], positions[bottom_right]];

            let normal_1 = calc_normal(tri_1[0], tri_1[1], tri_1[2]);
            let normal_2 = calc_normal(tri_2[0], tri_2[1], tri_2[2]);

            // Since a vertex can belong to multiple triangles, average the normals
            normals[top_left] = avg_normal(normals[top_left], normal_1);
            normals[bottom_left] = avg_normal(normals[bottom_left], normal_1);
            normals[top_right] = avg_normal(avg_normal(normals[top_right], normal_1), normal_2);
            normals[bottom_right] = avg_normal(normals[bottom_right], normal_2);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float32x3(positions));
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float32x3(normals));
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    mesh
}

fn calc_normal(p0: [f32; 3], p1: [f32; 3], p2: [f32; 3]) -> [f32; 3] {
    let v0 = Vec3::from(p1) - Vec3::from(p0);
    let v1 = Vec3::from(p2) - Vec3::from(p0);
    let normal = v0.cross(v1).normalize();
    normal.into()
}

fn avg_normal(n1: [f32; 3], n2: [f32; 3]) -> [f32; 3] {
    ((Vec3::from(n1) + Vec3::from(n2)) * 0.5).normalize().into()
}