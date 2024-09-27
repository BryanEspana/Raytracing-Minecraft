use bevy::prelude::*;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::window::WindowPlugin;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::prelude::Mesh;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280., 720.).into(), 
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, text_input)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 20.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y), // Ajuste de la cámara para mejor visualización
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0, // Aumentar la intensidad de la luz
            ..default()
        },
        transform: Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
    let texture_handle = asset_server.load("textures\\branches.png");

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    
    let vertices = vec![
        ([ -25.0,  0.0,  25.0], [0.0, 1.0, 0.0], [0.0, 0.0]), 
        ([  25.0,  0.0,  25.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([  25.0,  0.0, -25.0], [0.0, 1.0, 0.0], [1.0, 1.0]), 
        ([ -25.0,  0.0, -25.0], [0.0, 1.0, 0.0], [0.0, 1.0]), 
    ];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.iter().map(|(p, _, _)| *p).collect::<Vec<_>>());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertices.iter().map(|(_, n, _)| *n).collect::<Vec<_>>());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices.iter().map(|(_, _, uv)| *uv).collect::<Vec<_>>());

    mesh.insert_indices(Indices::U32(vec![
        0, 2, 1,
        0, 3, 2, 
    ]));

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle),
            ..default()
        }),
        ..default()
    });
}

fn text_input(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    for mut transform in query.iter_mut() {
        for ev in evr_kbd.read() {
            if ev.state == ButtonState::Released {
                continue;
            }

            match &ev.logical_key {
                Key::Character(input) => {
                    match input.as_str() {
                        "w" => transform.translation.z -= 1.0,
                        "s" => transform.translation.z += 1.0,
                        "a" => transform.translation.x -= 1.0,
                        "d" => transform.translation.x += 1.0,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
