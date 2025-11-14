use crate::{
    components::Tile, constants::{map::{CHUNK_SIZE, HEX_LAYOUT, MAP_RADIUS}, z_layers}, resources::chunk::{Chunk, Chunks}
};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    pbr::StandardMaterial,
    prelude::*,
};
use hexx::{ColumnMeshBuilder, Hex, HexLayout, PlaneMeshBuilder, hex, shapes};

pub const HEX_SIZE: Vec2 = Vec2::splat(64.0);
pub const COLUMN_HEIGHT: f32 = 10.0;
const COLORS: [Color; 3] = [
    Color::srgba(60. / 255., 60. / 255., 60. / 255., 1.),
    Color::srgba(65. / 255., 65. / 255., 65. / 255., 1.),
    Color::srgba(55. / 255., 55. / 255., 55. / 255., 1.),
];

pub fn generate_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunks: ResMut<Chunks>,
) {
    println!("generating tiles");

    let mesh = hexagonal_plane(&HEX_LAYOUT);
    let mesh_handle = meshes.add(mesh);

    let material_handles = [
        materials.add(StandardMaterial::from(COLORS[0])),
        materials.add(StandardMaterial::from(COLORS[1])),
        materials.add(StandardMaterial::from(COLORS[2])),
    ];

    for hex in shapes::hexagon(hex(0, 0), MAP_RADIUS) {
        
        commands.spawn(Tile::new(hex));
        
        if chunks.get_with_hex(&hex).is_some() {
            continue;
        };

        let chunk_hex = hex.to_lower_res(CHUNK_SIZE);

        for hex in shapes::hexagon(chunk_hex.to_higher_res(CHUNK_SIZE), CHUNK_SIZE) {
            generate_chunk(
                chunk_hex,
                hex,
                &mut commands,
                &material_handles,
                &mesh_handle,
                &mut materials,
                &mut chunks
            );
        }
    }
}

fn generate_chunk(
    chunk_hex: Hex,
    hex: Hex,
    commands: &mut Commands,
    material_handles: &[Handle<StandardMaterial>],
    mesh_handle: &Handle<Mesh>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunks: &mut ResMut<Chunks>
) {
    let pos = HEX_LAYOUT.hex_to_world_pos(hex);
    let color_index = (chunk_hex.x - chunk_hex.y).rem_euclid(3);
    let material_handle = material_handles[color_index as usize].clone();
    let offset = /* (50. + random::<f32>() * 3.) */55. / 255.;
    let _material_handle = materials.add(StandardMaterial::from(Color::srgba(
        offset, offset, offset, 1.,
    )));

    // let handle = materials.add(StandardMaterial::from(COLORS[0]));

    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        Transform::from_xyz(pos.x, pos.y, 0.0),
        MeshMaterial3d(material_handle),
    ));

    chunks.0.insert(chunk_hex, Chunk::new(chunk_hex));
}

pub fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, COLUMN_HEIGHT)
        // < 1 creates borders around hexes
        .with_scale(Vec3::splat(1.))
        .facing(Vec3::Z)
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
