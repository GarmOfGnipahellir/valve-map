use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    render::{
        mesh::Indices,
        render_resource::{AddressMode, FilterMode, PrimitiveTopology, SamplerDescriptor},
        texture::ImageSettings,
    },
};
use valve_map::{from_str, meshing::ToMesh};

fn main() {
    App::new()
        .insert_resource(ImageSettings {
            default_sampler: SamplerDescriptor {
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                address_mode_w: AddressMode::Repeat,
                mag_filter: FilterMode::Nearest,
                min_filter: FilterMode::Nearest,
                ..Default::default()
            },
        })
        .insert_resource(AssetServerSettings {
            asset_folder: "examples".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let map = from_str(include_str!("basic.map")).unwrap();
    let worldspawn = &map.entities[0];
    let entity_mesh = worldspawn.to_mesh().unwrap();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, entity_mesh.positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, entity_mesh.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, entity_mesh.uvs);
    mesh.set_indices(Some(Indices::U32(entity_mesh.indices)));

    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(-2.0, -1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("TECH28.png")),
            ..Default::default()
        }),
        ..Default::default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-300.0, -150.0, 150.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    });
}
