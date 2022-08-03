// (x1 y1 z1) (x2 y2 z2) (x3 y3 z3) TEXTURE_NAME [ ux uy uz offsetX ] [ vx vy vz offsetY ] rotation scaleX scaleY
pub struct Face {
    triangle: [[f32; 3]; 3],
    texture_name: String,
    axis_u: [f32; 3],
    axis_v: [f32; 3],
    offset: [f32; 2],
    rotation: f32,
    scale: [f32; 2],
}
