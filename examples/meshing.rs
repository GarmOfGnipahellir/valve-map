use macroquad::{models::Vertex, prelude::*};
use valve_map::{from_str, meshing};

#[macroquad::main("Valve Map")]
async fn main() {
    let map = from_str(include_str!("basic.map")).unwrap();
    let brush = &map.entities[0].brushes[0];
    let mesh = meshing::Mesh::from_brush(brush).unwrap();
    let mesh = Mesh {
        vertices: mesh
            .positions
            .iter()
            .map(|p| Vertex {
                position: Vec3::from_slice_unaligned(p),
                uv: Vec2::ZERO,
                color: WHITE,
            })
            .collect(),
        indices: mesh.indices.iter().map(|i| *i as u16).collect(),
        texture: None,
    };

    loop {
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: vec3(512.0, 256.0, 256.0),
            target: Vec3::ZERO,
            ..Default::default()
        });

        draw_mesh(&mesh);

        next_frame().await
    }
}
